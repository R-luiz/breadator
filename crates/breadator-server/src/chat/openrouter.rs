//! OpenRouter chat completions, streamed: the request the benchmark settled on
//! (`research/notes/chat-model/REPORT.md`) and the reading of its server-sent events.

use std::time::Duration;

use serde_json::{Value, json};

use super::ChatState;

/// Primary model, then the fallback OpenRouter switches to by itself.
pub const MODELS: [&str; 2] = ["z-ai/glm-5.3-flash", "deepseek/deepseek-v4.1-flash-1017"];
/// Output tokens of one call, reasoning included.
pub const MAX_TOKENS: i64 = 800;
/// Price ceilings (`provider.max_price`), USD per million tokens: no endpoint above them is used.
const MAX_PRICE_PROMPT: f64 = 0.08;
const MAX_PRICE_COMPLETION: f64 = 0.6;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    /// JSON text, as the model wrote it.
    pub arguments: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Usage {
    pub tokens_in: i64,
    pub tokens_out: i64,
    pub cost_usd: f64,
}

#[derive(Debug, Default)]
pub struct Completion {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    /// Sent back with the tool results: some models need their reasoning to continue.
    pub reasoning_details: Vec<Value>,
    pub model: String,
    pub provider: String,
}

pub struct Outcome {
    pub completion: Result<Completion, String>,
    /// What the call cost; `None` when nothing was billed (refused or never sent).
    pub usage: Option<Usage>,
}

/// Body of one call. `allow_tools` is false for the last call of an answer, which must be text.
pub fn request(messages: &[Value], tools: &Value, session_id: &str, allow_tools: bool) -> Value {
    json!({
        "models": MODELS,
        // Zero data retention only, cheapest endpoint first, never above the price ceilings.
        "provider": {
            "zdr": true,
            "sort": "price",
            "max_price": { "prompt": MAX_PRICE_PROMPT, "completion": MAX_PRICE_COMPLETION },
        },
        "reasoning": { "effort": "low" },
        "max_tokens": MAX_TOKENS,
        "usage": { "include": true },
        "stream": true,
        // Keeps a conversation on one endpoint, whose prompt cache it can then reuse.
        "session_id": session_id,
        "tools": tools,
        "tool_choice": if allow_tools { "auto" } else { "none" },
        "messages": messages,
    })
}

/// Cost of `tokens_in`/`tokens_out` at the price ceilings: the most a call can have cost.
fn ceiling_cost(tokens_in: i64, tokens_out: i64) -> f64 {
    (tokens_in as f64 * MAX_PRICE_PROMPT + tokens_out as f64 * MAX_PRICE_COMPLETION) / 1e6
}

/// What a call is counted at when its cost is not known: the whole request as prompt (about
/// three bytes per token) and a full output, at the price ceilings. Also what a call is reserved
/// at before it is made.
pub fn estimate(request_bytes: usize) -> Usage {
    let tokens_in = i64::try_from(request_bytes / 3).unwrap_or(i64::MAX);
    Usage { tokens_in, tokens_out: MAX_TOKENS, cost_usd: ceiling_cost(tokens_in, MAX_TOKENS) }
}

/// A `usage` object. Without a valid `cost`, the call is counted at the larger of its token
/// counts at the ceilings and `estimate`: an empty `usage` is never free.
fn usage(u: &Value, estimate: Usage) -> Usage {
    let tokens_in = u["prompt_tokens"].as_i64().unwrap_or(0);
    let tokens_out = u["completion_tokens"].as_i64().unwrap_or(0);
    match u["cost"].as_f64().filter(|c| c.is_finite() && *c >= 0.0) {
        Some(cost_usd) => Usage { tokens_in, tokens_out, cost_usd },
        None => {
            let reported =
                Usage { tokens_in, tokens_out, cost_usd: ceiling_cost(tokens_in, tokens_out) };
            if reported.cost_usd >= estimate.cost_usd { reported } else { estimate }
        }
    }
}

/// `message (code)` of an OpenRouter error object, kept short for the log.
fn error_text(error: &Value) -> String {
    let message = error["message"].as_str().unwrap_or("unknown error");
    let message: String = message.chars().take(300).collect();
    match &error["code"] {
        Value::Null => message,
        code => format!("{message} ({code})"),
    }
}

/// The stream of one call: `data: {chunk}` lines, with text, tool calls and reasoning arriving
/// in pieces and the usage in the last chunk.
#[derive(Default)]
struct Stream {
    /// Bytes of an unfinished line.
    pending: Vec<u8>,
    completion: Completion,
    usage: Option<Value>,
    error: Option<String>,
    chunks: usize,
}

impl Stream {
    fn feed(&mut self, bytes: &[u8], on_text: &mut (dyn FnMut(&str) + Send)) {
        let mut pending = std::mem::take(&mut self.pending);
        pending.extend_from_slice(bytes);
        let mut start = 0;
        while let Some(len) = pending[start..].iter().position(|b| *b == b'\n') {
            self.line(&pending[start..start + len], on_text);
            start += len + 1;
        }
        pending.drain(..start);
        self.pending = pending;
    }

    fn line(&mut self, line: &[u8], on_text: &mut (dyn FnMut(&str) + Send)) {
        // Anything else is a comment (`: OPENROUTER PROCESSING`) or a blank separator.
        let Some(data) = std::str::from_utf8(line).ok().and_then(|l| l.strip_prefix("data:"))
        else {
            return;
        };
        let Ok(chunk) = serde_json::from_str::<Value>(data.trim()) else { return };
        self.chunks += 1;
        if let Some(error) = chunk.get("error").filter(|e| !e.is_null()) {
            self.error = Some(error_text(error));
        }
        if let Some(model) = chunk["model"].as_str() {
            model.clone_into(&mut self.completion.model);
        }
        if let Some(provider) = chunk["provider"].as_str() {
            provider.clone_into(&mut self.completion.provider);
        }
        if let Some(u) = chunk.get("usage").filter(|u| u.is_object()) {
            self.usage = Some(u.clone());
        }
        for choice in chunk["choices"].as_array().into_iter().flatten() {
            let delta = &choice["delta"];
            if let Some(text) = delta["content"].as_str().filter(|t| !t.is_empty()) {
                self.completion.content.push_str(text);
                on_text(text);
            }
            for call in delta["tool_calls"].as_array().into_iter().flatten() {
                self.tool_call(call);
            }
            for detail in delta["reasoning_details"].as_array().into_iter().flatten() {
                self.reasoning(detail);
            }
        }
    }

    fn tool_call(&mut self, call: &Value) {
        let calls = &mut self.completion.tool_calls;
        let id = call["id"].as_str().filter(|id| !id.is_empty());
        let slot = match call["index"].as_u64() {
            Some(index) => usize::try_from(index).unwrap_or(usize::MAX),
            // No index: a new id starts a new call, anything else continues the last one.
            None => match (id, calls.last()) {
                (Some(id), Some(last)) if last.id != id => calls.len(),
                _ => calls.len().saturating_sub(1),
            },
        };
        // A model asking for more calls than this is not answering a bread question.
        if slot >= 16 {
            return;
        }
        while calls.len() <= slot {
            calls.push(ToolCall {
                id: String::new(),
                name: String::new(),
                arguments: String::new(),
            });
        }
        let entry = &mut calls[slot];
        if let Some(id) = id {
            id.clone_into(&mut entry.id);
        }
        let function = &call["function"];
        if let Some(name) = function["name"].as_str().filter(|_| entry.name.is_empty()) {
            name.clone_into(&mut entry.name);
        }
        if let Some(arguments) = function["arguments"].as_str() {
            entry.arguments.push_str(arguments);
        }
    }

    /// Reasoning blocks arrive in pieces sharing a `type` and an `index`: their text is joined.
    fn reasoning(&mut self, detail: &Value) {
        let same = |d: &&mut Value| d["type"] == detail["type"] && d["index"] == detail["index"];
        let Some(existing) = self.completion.reasoning_details.iter_mut().find(same) else {
            self.completion.reasoning_details.push(detail.clone());
            return;
        };
        let (Some(existing), Some(detail)) = (existing.as_object_mut(), detail.as_object()) else {
            return;
        };
        for (key, value) in detail {
            match (existing.get_mut(key), value) {
                (Some(Value::String(joined)), Value::String(piece))
                    if matches!(key.as_str(), "text" | "summary" | "data") =>
                {
                    joined.push_str(piece);
                }
                (Some(Value::Null) | None, value) if !value.is_null() => {
                    existing.insert(key.clone(), value.clone());
                }
                _ => {}
            }
        }
    }

    fn finish(mut self, estimate: Usage, on_text: &mut (dyn FnMut(&str) + Send)) -> Outcome {
        // A last line without its newline.
        let rest = std::mem::take(&mut self.pending);
        self.line(&rest, on_text);
        self.completion.tool_calls.retain(|call| !call.name.is_empty());
        let completion = match self.error {
            Some(error) => Err(error),
            None if self.chunks == 0 => Err("the response had no data".to_owned()),
            None => Ok(self.completion),
        };
        // An accepted call that never reported its cost is counted at the ceiling, never as free.
        let usage = self.usage.map_or(estimate, |u| usage(&u, estimate));
        Outcome { completion, usage: Some(usage) }
    }
}

/// One streamed call of `body` (a serialized [`request`]). `on_text` receives the answer text as
/// it arrives.
pub async fn call(
    chat: &ChatState,
    key: &str,
    referer: &str,
    body: Vec<u8>,
    timeout: Duration,
    on_text: &mut (dyn FnMut(&str) + Send),
) -> Outcome {
    let estimate = estimate(body.len());
    let sent = chat
        .http
        .post(format!("{}/chat/completions", chat.api_url))
        .bearer_auth(key)
        .header("content-type", "application/json")
        // How OpenRouter attributes the traffic in its dashboard.
        .header("http-referer", referer)
        .header("x-title", "Breadator")
        .timeout(timeout)
        .body(body)
        .send()
        .await;
    let mut response = match sent {
        Ok(response) => response,
        Err(e) => {
            // Never connected: nothing ran. Anything later may have run without us hearing of it.
            let usage = (!e.is_connect()).then_some(estimate);
            let completion = Err(format!("request failed: {}", e.without_url()));
            return Outcome { completion, usage };
        }
    };
    let status = response.status();
    if !status.is_success() {
        let text = response.text().await.unwrap_or_default();
        let message = serde_json::from_str::<Value>(&text)
            .ok()
            .and_then(|v| v.get("error").map(error_text))
            .unwrap_or_else(|| text.chars().take(300).collect());
        return Outcome { completion: Err(format!("HTTP {status}: {message}")), usage: None };
    }
    let mut stream = Stream::default();
    loop {
        match response.chunk().await {
            Ok(Some(bytes)) => stream.feed(&bytes, on_text),
            Ok(None) => break,
            Err(e) => {
                stream.error.get_or_insert(format!("stream interrupted: {}", e.without_url()));
                break;
            }
        }
    }
    stream.finish(estimate, on_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(body: &str, step: usize) -> (Outcome, String) {
        let mut stream = Stream::default();
        let mut text = String::new();
        let mut on_text = |t: &str| text.push_str(t);
        for piece in body.as_bytes().chunks(step) {
            stream.feed(piece, &mut on_text);
        }
        let outcome = stream.finish(estimate(3000), &mut on_text);
        (outcome, text)
    }

    const TOOL_STREAM: &str = concat!(
        ": OPENROUTER PROCESSING\n\n",
        "data: {\"model\":\"z-ai/glm-5.3-flash\",\"provider\":\"Relace\",\"choices\":[{\"delta\":{\"role\":\"assistant\",\"content\":\"\",\"reasoning_details\":[{\"type\":\"reasoning.text\",\"index\":0,\"text\":\"Need \"}]}}]}\n\n",
        "data: {\"choices\":[{\"delta\":{\"reasoning_details\":[{\"type\":\"reasoning.text\",\"index\":0,\"text\":\"weights.\"}]}}]}\n\n",
        "data: {\"choices\":[{\"delta\":{\"tool_calls\":[{\"index\":0,\"id\":\"call_a\",\"type\":\"function\",\"function\":{\"name\":\"compute_formula\",\"arguments\":\"\"}}]}}]}\n\n",
        "data: {\"choices\":[{\"delta\":{\"tool_calls\":[{\"index\":0,\"function\":{\"arguments\":\"{\\\"grams\\\":\"}}]}}]}\n\n",
        "data: {\"choices\":[{\"delta\":{\"tool_calls\":[{\"index\":0,\"function\":{\"arguments\":\"900}\"}},{\"index\":1,\"id\":\"call_b\",\"function\":{\"name\":\"search_knowledge\",\"arguments\":\"{\\\"queries\\\":[\\\"pâte\\\"]}\"}}]},\"finish_reason\":\"tool_calls\"}]}\n\n",
        "data: {\"choices\":[],\"usage\":{\"prompt_tokens\":3100,\"completion_tokens\":41,\"cost\":0.000113}}\n\n",
        "data: [DONE]\n\n",
    );

    #[test]
    fn tool_calls_reasoning_and_usage_arrive_in_pieces() {
        // Whatever the size of the network chunks, even one cutting a UTF-8 character in two.
        for step in [1, 7, 4096] {
            let (outcome, text) = read(TOOL_STREAM, step);
            let completion = outcome.completion.unwrap();
            assert_eq!(text, "");
            assert_eq!(completion.model, "z-ai/glm-5.3-flash");
            assert_eq!(completion.provider, "Relace");
            assert_eq!(
                completion.tool_calls,
                vec![
                    ToolCall {
                        id: "call_a".into(),
                        name: "compute_formula".into(),
                        arguments: "{\"grams\":900}".into()
                    },
                    ToolCall {
                        id: "call_b".into(),
                        name: "search_knowledge".into(),
                        arguments: "{\"queries\":[\"pâte\"]}".into()
                    },
                ]
            );
            assert_eq!(
                completion.reasoning_details,
                vec![json!({"type": "reasoning.text", "index": 0, "text": "Need weights."})]
            );
            assert_eq!(
                outcome.usage,
                Some(Usage { tokens_in: 3100, tokens_out: 41, cost_usd: 0.000113 })
            );
        }
    }

    #[test]
    fn text_is_passed_on_as_it_arrives() {
        let body = "data: {\"choices\":[{\"delta\":{\"content\":\"Le sel \"}}]}\n\ndata: {\"choices\":[{\"delta\":{\"content\":\"freine [1].\"}}],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":5,\"cost\":0.00001}}";
        let (outcome, text) = read(body, 5);
        assert_eq!(text, "Le sel freine [1].");
        assert_eq!(outcome.completion.unwrap().content, "Le sel freine [1].");
        assert_eq!(outcome.usage.unwrap().cost_usd, 0.00001);
    }

    #[test]
    fn errors_and_missing_costs() {
        // An error in the middle of the stream fails the call; its unknown cost is the estimate.
        let body = "data: {\"choices\":[{\"delta\":{\"content\":\"Le\"}}]}\n\ndata: {\"error\":{\"code\":502,\"message\":\"Provider disconnected\"},\"choices\":[{\"delta\":{},\"finish_reason\":\"error\"}]}\n\n";
        let (outcome, _) = read(body, 64);
        assert_eq!(outcome.completion.unwrap_err(), "Provider disconnected (502)");
        let counted = outcome.usage.unwrap();
        assert_eq!((counted.tokens_in, counted.tokens_out), (1000, MAX_TOKENS));
        assert!((counted.cost_usd - 0.00056).abs() < 1e-12, "{}", counted.cost_usd);

        // Usage without a cost is priced at the ceilings...
        let body = "data: {\"choices\":[{\"delta\":{\"content\":\"ok\"}}],\"usage\":{\"prompt_tokens\":1000000,\"completion_tokens\":1000000}}\n\n";
        let (outcome, _) = read(body, 64);
        assert!((outcome.usage.unwrap().cost_usd - 0.68).abs() < 1e-9);
        // ...or at the request's estimate when that is more: an empty or token-less usage is not
        // free.
        for usage in ["{}", "{\"prompt_tokens\":10,\"completion_tokens\":2}", "{\"cost\":null}"] {
            let body = format!(
                "data: {{\"choices\":[{{\"delta\":{{\"content\":\"ok\"}}}}],\"usage\":{usage}}}\n\n"
            );
            let (outcome, _) = read(&body, 64);
            assert_eq!(outcome.usage, Some(estimate(3000)), "{usage}");
        }

        let (outcome, _) = read(": ping\n\n", 64);
        assert!(outcome.completion.is_err());
    }

    #[test]
    fn request_is_the_benchmarked_one() {
        let body = request(&[json!({"role": "user", "content": "q"})], &json!([]), "conv", true);
        assert_eq!(
            body["models"],
            json!(["z-ai/glm-5.3-flash", "deepseek/deepseek-v4.1-flash-1017"])
        );
        assert_eq!(
            body["provider"],
            json!({"zdr": true, "sort": "price", "max_price": {"prompt": 0.08, "completion": 0.6}})
        );
        assert_eq!(body["reasoning"], json!({"effort": "low"}));
        assert_eq!(body["max_tokens"], 800);
        assert_eq!(body["usage"], json!({"include": true}));
        assert_eq!(body["stream"], true);
        assert_eq!(body["tool_choice"], "auto");
        assert_eq!(request(&[], &json!([]), "conv", false)["tool_choice"], "none");
    }
}
