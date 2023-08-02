
use uuid::Uuid;
use rust_decimal::prelude::*;

pub struct Chat {
    pub id: Uuid,
    pub sender: String,
    pub receiver: String,
    pub created_date: chrono::NaiveDateTime,
    pub content: String,
    pub amount: Decimal,
}
