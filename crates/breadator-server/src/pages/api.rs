//! JSON API: the engine over HTTP for the chatbot's tools and any client without WASM.
//! Replies use the same shape as the WASM bindings: `{"ok": …}` or `{"error": {"code": …}}`.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use breadator_engine::EngineError;
use breadator_engine::formula::Formula;
use breadator_engine::ingredients::IngredientKind;
use breadator_engine::temperature::WaterTempInput;
use serde::{Deserialize, Serialize};

fn reply<T: Serialize>(result: Result<T, EngineError>) -> Response {
    match result {
        Ok(v) => Json(serde_json::json!({ "ok": v })).into_response(),
        Err(e) => (StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({ "error": e })))
            .into_response(),
    }
}

pub async fn formula(Json(f): Json<Formula>) -> Response {
    reply(breadator_engine::formula::solve(&f))
}

pub async fn water_temperature(Json(i): Json<WaterTempInput>) -> Response {
    reply(breadator_engine::temperature::water_temperature(&i))
}

#[derive(Deserialize)]
pub struct YeastInput {
    grams: f64,
    from: IngredientKind,
    to: IngredientKind,
}

pub async fn yeast(Json(i): Json<YeastInput>) -> Response {
    reply(breadator_engine::yeast::convert(i.grams, i.from, i.to))
}

pub async fn fermentation(Json(q): Json<breadator_engine::leaven::QuickFerment>) -> Response {
    reply(q.solve())
}

pub async fn starter(Json(q): Json<breadator_engine::leaven::QuickStarter>) -> Response {
    reply(q.solve())
}

pub async fn flour(Json(q): Json<breadator_engine::flour::QuickFlour>) -> Response {
    reply(q.solve())
}
