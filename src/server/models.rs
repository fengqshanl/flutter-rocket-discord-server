use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ServerResponse {
    pub id: String,
    pub name: String,
    pub url: String,
    pub img: String
}


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Server<'r> {
    pub name: &'r str,
    pub url: &'r str,
    pub img: &'r str
}