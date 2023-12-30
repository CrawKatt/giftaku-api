use once_cell::sync::Lazy;
use rocket::fs::FileServer;
use rocket::{launch, routes};
use std::env;
use surrealdb::engine::local::{Db, File};
use surrealdb::Surreal;

mod routes;
use crate::routes::gets::{get_endpoints, get_gif, index, login, register, send_result};
use crate::routes::posts::upload;

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);
pub type RocketResult<T> = Result<T, rocket::response::status::BadRequest<String>>;
pub static URL_HOST: Lazy<String> =
    Lazy::new(|| dotenvy::var("DEPLOY_ADDRESS").unwrap_or_else(|_| String::from("localhost")));

#[launch]
async fn rocket() -> _ {
    let db_path = env::current_dir().unwrap_or_default().join("./migrations");
    DB.connect::<File>(db_path).await.unwrap_or_else(|why| {
        panic!("Could not connect to database: {why}");
    });

    rocket::build()
        .mount(
            "/",
            routes![
                upload,
                index,
                login,
                register,
                send_result,
                get_gif,
                get_endpoints
            ],
        )
        .mount("/static", FileServer::from("static/"))
}
