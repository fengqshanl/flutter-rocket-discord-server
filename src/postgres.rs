use rocket::{fairing::{AdHoc}, serde::__private::de};
use crate::server::{handlers::{get_server, create_server, update_server, delete_server}};
use rocket_db_pools::{deadpool_postgres, Database, Connection};
#[derive(Database)]
#[database("discord")]
pub struct Discord(deadpool_postgres::Pool);

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "ру")]
    Russian
}
#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

#[get("/?<lang>&<opt..>")]
fn hello(_db: Connection<Discord> ,lang: Option<Lang>, opt: Options<'_>) -> String {
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("👋 ");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(Discord::init())
            .mount("/", routes![hello])
            .mount("/server", routes![get_server, create_server, update_server, delete_server])
    })
}