use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct Reservation {
    pub ref_no: Uuid,
    pub hotel_name: String,
    pub no_of_rooms: i16,
    pub check_in_date: NaiveDate,
    pub check_out_date: NaiveDate,
    pub reservation_time: NaiveDateTime,
    pub customer_name: String,
    pub total_amount: f32,
}

pub struct LagoonDb<'a> {
    pool: &'a PgPool,
}

impl<'a> LagoonDb<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn make_reservation(
        &self,
        reservation: Reservation,
    ) -> Result<Reservation, sqlx::Error> {
        sqlx::query_as::<_, Reservation>(
            "INSERT INTO reservation (ref_no, hotel_name, no_of_rooms, check_in_date, check_out_date, reservation_time, customer_name, total_amount) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
        )
            .bind(reservation.ref_no)
            .bind(reservation.hotel_name)
            .bind(reservation.no_of_rooms)
            .bind(reservation.check_in_date)
            .bind(reservation.check_out_date)
            .bind(reservation.reservation_time)
            .bind(reservation.customer_name)
            .bind(reservation.total_amount)
            .fetch_one(self.pool)
            .await
    }
}
