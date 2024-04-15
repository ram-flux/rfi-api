//
//  Copyright 2024 Ram Flux, LLC.
//

pub async fn handler_404() -> impl axum::response::IntoResponse {
    let jsons = serde_json::json!({
        "code": 404,
        "msg": "error 4004",
        "result":""
    });

    (axum::http::StatusCode::NOT_FOUND, axum::Json(jsons))
}
