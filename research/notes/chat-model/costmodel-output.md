Token budget per call (generic tokens, x tokenizer factor): {"one":[[3550,2500,80],[6630,3630,500]],"two":[[3550,2500,80],[6630,3630,80],[7010,6710,500]]}; reference first-call prompt = 1972 tokens
| Model | Route priced | $/M in / out | $/M cache read | Tokenizer factor | Measured reasoning tok/answer | Measured m$/answer (benchmark, ~4k-token context) | m$ per answer, 1 / 2 tool trips, no cache | m$ per answer, 1 / 2 trips, cached | Answers per $1 (2 trips, no cache) | Answers per $1 (2 trips, cached) |
|---|---|---|---|---|---|---|---|---|---|---|
| deepseek/deepseek-v4.1-flash-1017 | observed: Wafer (wafer/us) | 0.036 / 0.500 | 0.0100 | 0.96 | 60 | 0.386 | 0.662 / 0.943 | 0.509 / 0.622 | 1,059 | 1,607 |
| anthropic/claude-haiku-5.5 | observed: Google (google-vertex/global) | 0.100 / 0.500 | 0.0100 | 1.39 | 88 | 1.057 | 1.864 / 2.895 | 1.237 / 1.438 | 345 | 695 |
| openai/gpt-6-luna | observed: Azure (azure) | 0.100 / 0.500 | 0.0100 | 0.72 | 0 | 0.325 | 0.947 / 1.483 | 0.621 / 0.726 | 674 | 1,378 |
| meta/muse-ember | observed: Google (google-vertex/global) | 0.100 / 0.500 | 0.0040 | 1.04 | 179 | 0.524 | 1.451 / 2.223 | 0.839 / 0.940 | 449 | 1,064 |
| z-ai/glm-5.3-flash | observed: Relace (relace/fp4) | 0.036 / 0.081 | 0.0300 | 0.88 | 4 | 0.149 | 0.361 / 0.587 | 0.331 / 0.523 | 1,704 | 1,910 |
| inclusionai/ling-3.1-flash | no ZDR endpoint: list price | 0.075 / 0.225 | 0.0150 | 1.00 | 0 | - | 0.894 / 1.438 | 0.526 / 0.667 | 695 | 1,498 |
| qwen/qwen3.8-flash | no ZDR endpoint: list price | 0.150 / 0.470 | 0.0160 | 1.00 | 0 | - | 1.800 / 2.889 | 0.978 / 1.168 | 346 | 856 |
| deepseek/deepseek-v4-flash | cheapest ZDR: Relace (relace/fp4) | 0.050 / 0.450 | 0.0500 | 1.00 | 0 | - | 0.770 / 1.157 | 0.770 / 1.157 | 864 | 864 |
| xiaomi/mimo-v2.6-flash | cheapest ZDR: Cohere (cohere/fp8) | 0.140 / 0.280 | 0.0030 | 1.00 | 0 | - | 1.588 / 2.591 | 0.748 / 0.832 | 385 | 1,201 |
