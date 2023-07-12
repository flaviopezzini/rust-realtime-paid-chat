
use uuid::Uuid;

pub struct Chat {
    pub id: Uuid,
    pub sender: String,
    pub receiver: String,
    pub created_date: chrono::NaiveDateTime,
    pub content: String
}