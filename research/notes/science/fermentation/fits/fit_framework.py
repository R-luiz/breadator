"""Compare dose-v-temperature fits to Framework Table 7.1, not measured durations.
Outputs fitted and leave-one-out residuals on dose (%) scale; stdlib only.
CC BY-SA 4.0 data: Hendrik Kleinwächter, The Sourdough Framework, table 7.1.
"""
import csv
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
DATA = {'recently_fed': [(20, 15), (25, 10), (30, 5)],
        'starving': [(20, 10), (25, 5), (30, 2.5)]}

def fit(points, exponential):
    xs = [x for x, _ in points]
    ys = [math.log(y) if exponential else y for _, y in points]
    xm, ym = sum(xs)/len(xs), sum(ys)/len(ys)
    slope = sum((x-xm)*(y-ym) for x,y in zip(xs,ys))/sum((x-xm)**2 for x in xs)
    return ym-slope*xm, slope

def predict(coeff, x, exponential):
    y = coeff[0]+coeff[1]*x
    return math.exp(y) if exponential else y

rows=[]
summary={}
for condition, data in DATA.items():
    summary[condition]={}
    for model, exp in [('linear',False),('exponential',True)]:
        co=fit(data,exp)
        for i,(x,y) in enumerate(data):
            pred=predict(co,x,exp)
            hold=predict(fit(data[:i]+data[i+1:],exp),x,exp)
            rows.append(dict(condition=condition,model=model,temp_c=x,observed_pct=y,predicted_pct=pred,residual_pct=pred-y,loocv_predicted_pct=hold,loocv_residual_pct=hold-y,source='The Sourdough Framework table 7.1'))
        subset=rows[-len(data):]
        summary[condition][model]={'intercept':co[0],'slope_per_c':co[1], 'in_sample_rmse_pct':math.sqrt(sum(r['residual_pct']**2 for r in subset)/len(data)), 'loocv_rmse_pct':math.sqrt(sum(r['loocv_residual_pct']**2 for r in subset)/len(data))}
with (HERE/'framework-residuals.csv').open('w',newline='',encoding='utf-8') as f:
    w=csv.DictWriter(f,fieldnames=rows[0]); w.writeheader(); w.writerows(rows)
(HERE/'framework-fit.json').write_text(json.dumps(summary,indent=2)+'\n',encoding='utf-8')
print(json.dumps(summary,indent=2))
