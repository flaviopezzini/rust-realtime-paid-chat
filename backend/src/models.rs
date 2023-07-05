use diesel::{prelude::*};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::chat)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chat {
    pub id: Uuid,
    pub sender: String,
    pub receiver: String,
    pub created_date: chrono::NaiveDateTime,
    pub content: String
}