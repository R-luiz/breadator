# BakingSmart (AndroXplorer): features and XML data model

Source: novitalistyani.com home page and `/recipes/` pages (fetched 2026-11, raw in `raw/web/novita/pages/`), plus the 10 recipe XML files in `raw/web/novita/xml/`. All rights reserved: this note describes features and structure for design input. It copies no UI text.

## Product facts (site, home page)
- Android app, one-time purchase (USD 2.99), no subscription. The site says there are no ads or social feeds and no account is required (privacy-first, local "vault").
- Advertised features:
  1. Fast entry with automatic hydration, baker's percentages and total weight.
  2. Dynamic scaling of yield for any pan or batch size.
  3. Bake-session analytics: each bake is logged and sessions are compared on timeline charts.
  4. "Studio mode": exports a shareable recipe image with custom text.
  5. Privacy: no mandatory cloud account.
- Distribution: each recipe page offers an XML download, which the app imports by file or by URL ("Import XML" / "Import from URL"). Some of these recipes (e.g. Rye Shokupan) come pre-installed as samples. The recipe pages are dated April 2026, and several say "Updated Apr 2026". These are newer than the videos and change their methods (see below).

## XML data model (as observed in all 10 files)
```
<recipes>
  <recipe id=UUID dateModified=epoch-ms>
    <title/> <description/> <imageUrl/>
    <prepTime>"3h 30m"</prepTime> <cookTime>"35m"</cookTime>   (free-text durations; may be empty)
    <servings>int</servings> <categoryId>int</categoryId> <scaleFactor>1.0</scaleFactor>
    <youtubeVideoId>id[&t=seconds]</youtubeVideoId>              (deep link to the method segment)
    <ingredientSections>
      <section title="Tangzhong|Poolish|Levain|Main Dough|Filling and Topping|Baking Pan 21x11x12 cm3">
        <ingredient name amount bakerPercentage="20.0%" [extraLiquidPercentage="87.0"]/>
      </section>…
    </ingredientSections>
    <instructions>
      <step text [extraInfo] [timerDurationMinutes] [extraImage]/>…
    </instructions>
  </recipe>
</recipes>
```
Observations that matter for Breadator's engine:
- **Sections are the stages.** A preferment, a scald and the final dough are separate sections. The final dough refers to an earlier stage through a zero-amount pseudo-ingredient ("All of the poolish", `amount=""`, `bakerPercentage="0.0%"`). One recipe nests stages: in Rye Shokupan the tangzhong goes into the levain section, and the levain goes into the main dough.
- **Baker's % uses total flour across every section**, starter flour included. Example: Rye Shokupan has 70 g rye in the tangzhong, 30 g rice in the tangzhong, 70 g rye in the levain, 480 g bread flour, and 45 g of flour inside a 90 g starter, for 694.7 g total flour. The starter's `bakerPercentage` of 6.4% is its flour share (45/694.7), not its weight.
- **`extraLiquidPercentage` = the share of an ingredient's weight that counts as water in hydration.** Values used: water and boiling water 100; milk and whole milk 87; egg 75; butter 16 or 17.05; honey 17; starter 50.3 (≈100%-hydration starter); flour, sugar, salt and yeast 0 or absent. The app's hydration therefore counts butter and honey water. That puts its figures about 1–2.5 points above a water + milk + egg calculation (for example, Snowy Shokupan is 67.1% by the app method vs 64.5%). Breadator should make these factors explicit, editable defaults.
- **Amounts are strings** and sometimes carry units ("30 g", "45 g"). The `bakerPercentage` strings carry "%". Unquantified items (toppings, semolina for dusting) have an empty amount.
- **A section can hold metadata instead of ingredients.** "Baking Pan 21x11x12 cm3" is an empty section used to record the pan size that the scaling feature works from.
- **Steps carry timers** (`timerDurationMinutes`, also used for the overnight fridge rest: 480) and optional `extraInfo` with the rationale. In every recipe the rationale ties the timings to a 27–28 °C kitchen and tells cooler kitchens to extend them. No temperature field exists, so temperature appears only in the text.
- No model of fermentation temperature, yeast-vs-time, dough temperature, scald temperature or flour properties. Scaling is linear.

## Method changes visible in the 2026 XML vs the videos
- Poolish: 1 h at room temperature, then the fridge overnight. Previously it stayed on the counter (Basic Asian Bread, Snowy Shokupan, Ciabatta, Pita, Shio Pan, Sub Rolls, Pizza).
- No extra instant yeast in the final dough when there is a mature overnight poolish. Bulk takes longer as a result (Basic Asian Bread 45–60 min to 1.5×; Snowy Shokupan 1–1.5 h).
- Snowy Shokupan is baked at 160 °C (25 min top+bottom heat, then 20 min bottom heat only), which differs from the video.
- Tangzhong is still made the night before in every XML (overnight fridge), consistent with the 24 h rule in ocBV6-O5SL4.

## Design input for Breadator
- Represent stages as first-class nodes (scald, preferment, levain, soaker, final), with references, nesting, and flour/water rolled up into totals.
- Treat ingredient water fractions as data: user-editable, with defaults water 1.0, milk 0.87, egg 0.75, butter 0.16, honey 0.17, starter by its hydration.
- Attach timers to steps, and attach a temperature to each timer. Breadator's fermentation model should rescale the timers that BakingSmart only annotates with "at 27 °C".
- Import/export: supporting BakingSmart XML import would let users bring these recipes in. The format has no license, so the files themselves must not be republished.
