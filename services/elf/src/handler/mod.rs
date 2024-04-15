//
//  Copyright 2024 Ram Flux, LLC.
//

mod parameter;

// use axum::{Extension, Json};
// use axum_extra::extract::WithRejection;

pub async fn init(
    axum::Extension(_app): axum::Extension<models::ArcLockAppState>,
    // axum_client_ip::SecureClientIp(client_ip): axum_client_ip::SecureClientIp,
    // WithRejection(Json(from), _): WithRejection<Json<parameter::DeviceInit>, common::ApiError>,
) -> Result<String, common::ApiError> {
    // let app = app.read().await;
    // let db = &app.db;
    // let rd = &app.rd;

    Ok("Hi...".to_string())
}
