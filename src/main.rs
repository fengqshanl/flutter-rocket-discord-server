#[macro_use] extern crate rocket;
pub mod postgres;
pub mod server;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(postgres::stage())
        
}
