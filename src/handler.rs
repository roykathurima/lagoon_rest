use axum::Json;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub async fn make_reservation(Json(payload): Json<RekwestPayload>) -> Json<Wesponse> {
    // How do we derive the response given the reqwest'
    let RekwestPayload {
        hotel_name,
        number_of_rooms,
        check_in_date,
        check_out_date,
        customer_name,
        reservation_time,
    } = payload;
    // TODO: Add these fields to the database just because
    Json(Wesponse {
        ref_no: Uuid::new_v4(),
        hotel_name,
        total_amount_payable: (number_of_rooms as f32) * 50.0,
        check_in_date,
        check_out_date,
        estimated_confirmation_time: "5 hours".into(),
    })
}
