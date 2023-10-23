use rocket::tokio::fs::File;
use rocket::serde::json::Json;
use std::path::{Path, PathBuf};
use rocket::{FromForm, get, launch, routes};
use tokio::io::AsyncReadExt;

#[derive(Debug, PartialEq, FromForm)]
struct Gif<'r> {
    name: &'r str,
    anime: &'r str,
    url: &'r str,
    gif_id: i32,
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, slap, files])
}

#[get("/<file>")]
async fn files(file: PathBuf) -> Option<Json<String>> {
    let mut buffer = Vec::new();
    let mut file = File::open(Path::new("static/").join(file)).await.ok()?;
    let file_name = file.read_to_end(&mut buffer).await.ok()?;
    let url = format!("data:image/gif/{}", file_name);
    Some(Json(url))
}

#[get("/slap")]
fn slap() -> &'static str {
    "Slap!"
}

#[get("/api/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {name}!")
}