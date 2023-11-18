use rocket::{launch, routes};
use rocket::fs::FileServer;
use surrealdb::engine::local::{Db, File};
use surrealdb::Surreal;
use once_cell::sync::Lazy;

mod routes;
use crate::routes::posts::upload;
use crate::routes::gets::{index, send_result, get_gif, get_endpoints};

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);
pub type RocketResult<T> = Result<T, rocket::response::status::BadRequest<String>>;

#[launch]
async fn rocket() -> _ {
    let url_host = dotenvy::var("URL_HOST");
    println!("URL_HOST: {url_host:?}");

    let db_path = std::env::current_dir().unwrap_or_default().join("./migrations");
    DB.connect::<File>(db_path).await.unwrap_or_else(|why| {
        panic!("Could not connect to database: {why}");
    });

    rocket::build()
        .mount("/", routes![upload, index, send_result, get_gif, get_endpoints])
        .mount("/static", FileServer::from("static/"))
}

