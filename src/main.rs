use rocket::{launch, routes};
use rocket::fs::FileServer;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use once_cell::sync::Lazy;

mod routes;
use crate::routes::posts::{upload, upload_slap};
use crate::routes::gets::{files, index, random_gif, slap};

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[launch]
async fn rocket() -> _ {
    DB.connect::<Mem>(()).await.unwrap_or_else(|why| {
        panic!("Could not connect to database: {why}");
    });

    rocket::build()
        .mount("/", routes![upload, files, index, random_gif, upload_slap, slap])
        .mount("/static", FileServer::from("static/"))
}

