use crate::database::establish_connection;

pub async fn save(chat: Chat) -> Result<(), Err> {
    let dbConn = establish_connection();

}