# Grading rubric, fixed before any live call

Each answer gets five criteria scored 0–2, so 10 points per question and 120 per model. One grader (the agent) reads the full transcript of each answer in `results/<model>.json`: the tool calls and their arguments, the tool results, and the final text. The script's automatic checks in `results/<model>.json → auto` are inputs to the grading, not the grade.

| Criterion | 2 | 1 | 0 |
|---|---|---|---|
| **Correctness** | All key facts in `prompts.json → key_facts` that the question needs are present and right. Numbers equal the tool output, rounded to the nearest gram or 0.1 h at most. | A minor fact is missing or vague, or a small inaccuracy that would not mislead a baker. | A wrong mechanism, a wrong number, an invented number presented as a fact, or no final answer. |
| **Tool use** | Mechanism and grounding questions: `search_knowledge` called before answering. Numeric questions: the expected engine tool is called with exactly the expected arguments (°F→°C within ±0.1; optional arguments may be omitted when the user did not give them), once per scenario, and the answer does no arithmetic of its own. | Tool called, but one argument is wrong or missing, a required call is skipped and the answer says so, or calls are clearly redundant (more than 2 searches). | No call where one is needed, or the model computes or guesses the numbers itself. |
| **Citation fidelity** | Every `[key]` cited was returned in this conversation and states the claim it is attached to. The irrelevant passage is not cited. Claims that go beyond the passages carry `[non sourcé]` / `[not sourced]`. On Q09 and Q12 the specifically unsourced part (rye core temperature; medical claims beyond the passages) is labeled. On numeric questions: tool numbers are not given fake citation keys, and added advice is labeled or sourced. | One mis-attached citation, or one unlabeled unsourced claim that is minor. | An invented key, the irrelevant passage cited as support, an unlabeled unsourced number, or no citations at all on a mechanism or grounding question. |
| **Language quality** | The answer is entirely in the question's language, idiomatic, and uses correct baking vocabulary (FR: pointage, apprêt, levain, mie…). | Understandable, but has awkward calques, a stray word in the other language, or a wrong term. | Wrong language, or hard to read. |
| **Brevity** | At most 250 words (tables count as words), with no filler. | 251–400 words, or some padding. | More than 400 words, or the answer rambles or repeats itself. |

## Derived metrics, recorded but not scored

- Cost per answer (`usage.cost`, summed over every call in the turn), prompt, completion and reasoning tokens, cached tokens, and number of LLM calls.
- Wall-clock latency per answer.
- Route used (`provider` in the response).

## Tie-breaks for the recommendation

Highest total score first. Within 5 points, the cheaper projected cost per answer under the production token budget wins. A model that scores 0 on tool use for any numeric question, or invents a citation key, cannot be primary.
