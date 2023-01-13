use rocket::serde::json::Json;
use uuid::Uuid;
use rocket_db_pools::{Connection};
use crate::postgres::Discord;
use super::models::{ ServerResponse, Server };

#[post("/", data = "<server>")]
pub async fn create_server(db: Connection<Discord>, server: Json<Server<'_>>) -> String{
    let id = Uuid::new_v4();
    let result = db.query("INSERT INTO server (serverId, serverName, serverUrl, serverImg) VALUES ($1, $2, $3, $4)", &[
        &id, &server.name, &server.img, &server.url
    ]).await;
    match result {
        Err(_) => return "insert error".to_string(),
        Ok(_) => return "insert success".to_string()
    }
}

#[get("/")]
pub async fn get_server(db: Connection<Discord>) -> Json<Vec<ServerResponse>> {
    let result = db.query("SELECT * FROM SERVER", &[]).await;
    match result {
       Err(_) => Json(vec![]),
       Ok(data) => {
            let value: Vec<ServerResponse> = data.iter().map(|row|{
                ServerResponse{
                    id: row.try_get::<&str, Uuid>("serverId").expect("msg").hyphenated().to_string(),
                    url: row.try_get::<&str, String>("serverUrl").expect("msg").to_string(),
                    name: row.try_get::<&str, String>("serverName").expect("msg").to_string(),
                    img: row.try_get::<&str, String>("serverImg").expect("msg").to_string(),
                }
            }).collect();
            Json(value)
        }
    }
}

#[put("/", data = "<server>")]
pub async fn update_server(db: Connection<Discord>, server: Json<ServerResponse>) -> String{
    let result = db.query("UPDATE SERVER SET serverName = $1, serverUrl = $2, serverImg = $3 WHERE serverId = $4", &[
        &server.name, &server.img, &server.url, &(Uuid::try_parse(&server.id).expect("msg"))
    ]).await;
    match result {
        Err(_) => return "update error".to_string(),
        Ok(_) => return "update success".to_string()
    }
}

#[put("/", data = "<id>")]
pub async fn delete_server(db: Connection<Discord>, id: Json<String>) -> String{
    let result = db.query("DELETE FROM SERVER WHERE serverId = $1", &[
        &(Uuid::try_parse(&id).expect("msg"))
    ]).await;
    match result {
        Err(_) => return "delete error".to_string(),
        Ok(_) => return "delete success".to_string()
    }
}