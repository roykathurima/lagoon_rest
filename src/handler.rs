use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

// local imports
use crate::db::{LagoonDb, Reservation};

#[derive(Deserialize)]
pub struct RekwestPayload {
    hotel_name: String,
    number_of_rooms: i8,
    check_in_date: NaiveDate,
    check_out_date: NaiveDate,
    customer_name: String,
    reservation_time: NaiveDateTime,
}

#[derive(Serialize)]
pub struct Wesponse {
    ref_no: Uuid,
    hotel_name: String,
    total_amount_payable: f32,
    check_in_date: NaiveDate,
    check_out_date: NaiveDate,
    estimated_confirmation_time: String,
}

// Define price per room as a constant
const PRICE_PER_ROOM_PER_NIGHT: f32 = 50.0;

pub async fn make_reservation(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(payload): Json<RekwestPayload>,
) -> impl IntoResponse {
    // How do we derive the response given the reqwest'
    let RekwestPayload {
        hotel_name,
        number_of_rooms,
        check_in_date,
        check_out_date,
        customer_name,
        reservation_time,
    } = payload;
    let duration = check_out_date - check_in_date;
    let days = duration.num_days();
    // Checkin date should always come before the checkout date;
    // Also, you cannot check in and checkout in the same day
    if days <= 0 {
        // Return an error response with HTTP 400
        let error_body = Json(json!({"error": "checkout date must come after the check in date"}));
        return (StatusCode::BAD_REQUEST, error_body).into_response();
    }
    let response = Wesponse {
        ref_no: Uuid::new_v4(),
        hotel_name: hotel_name.clone(),
        total_amount_payable: (number_of_rooms as f32) * PRICE_PER_ROOM_PER_NIGHT * (days as f32),
        check_in_date,
        check_out_date,
        estimated_confirmation_time: "5 hours".into(),
    };
    // Throw this in the DB just because
    let db = LagoonDb::new(&pool);
    let _ = db
        .make_reservation(Reservation {
            ref_no: response.ref_no,
            hotel_name,
            no_of_rooms: number_of_rooms,
            check_in_date,
            check_out_date,
            customer_name,
            reservation_time,
            total_amount: response.total_amount_payable,
        })
        .await;
    Json(response).into_response()
}
