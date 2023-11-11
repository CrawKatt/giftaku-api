use rocket::{launch, routes};
use rocket::fs::FileServer;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use once_cell::sync::Lazy;

mod routes;
use crate::routes::posts::upload;
use crate::routes::gets::{index, get_gif};

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[launch]
async fn rocket() -> _ {
    DB.connect::<Mem>(()).await.unwrap_or_else(|why| {
        panic!("Could not connect to database: {why}");
    });

    rocket::build()
        .mount("/", routes![upload, index, get_gif])
        .mount("/static", FileServer::from("static/"))
}

