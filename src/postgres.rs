use rocket::fairing::{AdHoc};
use rocket::serde::{Deserialize, json::Json};
use rocket_db_pools::{deadpool_postgres, Database, Connection};
use uuid::Uuid;
#[derive(Database)]
#[database("discord")]
struct Discord(deadpool_postgres::Pool);

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

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world(_db: Connection<Discord>) -> &'static str {
    "Hello, world!"
}

// Try visiting:
//   http://127.0.0.1:8000/hello/мир
#[get("/мир")]
fn mir() -> &'static str {
    "Привет, мир!"
}

// Try visiting:
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<age>")]
async fn wave(db: Connection<Discord> ,name: &str, age: u8) -> String {
    let result = db.query("SELECT * FROM SERVER", &[]).await.expect("query error");
    format!("👋 Hello, {} year old named {} result:{:?}!", age, name, result)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Server<'r> {
    name: &'r str,
    url: &'r str,
    img: &'r str
}

#[post("/", data = "<server>")]
async fn create_server(db: Connection<Discord>, server: Json<Server<'_>>) -> String{
    let id = Uuid::new_v4();
    let result = db.query("INSERT INTO server (serverId, serverName, serverUrl, serverImg) VALUES ($1, $2, $3, $4)", &[
        &id, &server.name, &server.img, &server.url
    ]).await;
    match result {
        Err(_) => return "insert error".to_string(),
        Ok(_) => return "insert success".to_string()
    }
}

// Note: without the `..` in `opt..`, we'd need to pass `opt.emoji`, `opt.name`.
//
// Try visiting:
//   http://127.0.0.1:8000/?emoji
//   http://127.0.0.1:8000/?name=Rocketeer
//   http://127.0.0.1:8000/?lang=ру
//   http://127.0.0.1:8000/?lang=ру&emoji
//   http://127.0.0.1:8000/?emoji&lang=en
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en
//   http://127.0.0.1:8000/?emoji&name=Rocketeer
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en&emoji
//   http://127.0.0.1:8000/?lang=ru&emoji&name=Rocketeer
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

// async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
//     match Discord::fetch(&rocket) {
//         Some(db) => match sqlx::migrate!("db/sqlx/migrations").run(&**db).await {
//             Ok(_) => Ok(rocket),
//             Err(e) => {
//                 error!("Failed to initialize SQLx database: {}", e);
//                 Err(rocket)
//             }
//         }
//         None => Err(rocket),
//     }
// }

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(Discord::init())
            // .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/", routes![hello])
            .mount("/hello", routes![world, mir])
            .mount("/wave", routes![wave])
            .mount("/create", routes![create_server])
    })
}