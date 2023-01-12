#[macro_use] extern crate rocket;
pub mod postgres;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(postgres::stage())
        
}
