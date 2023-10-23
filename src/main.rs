use rocket::tokio::fs::File;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use std::path::{Path, PathBuf};
use rocket::{FromForm, get, launch, routes};
use rocket::fs::FileServer;
use tokio::io::AsyncReadExt;

#[derive(Serialize)]
struct GifResponse {
    action: String,
    anime: String,
    gif_id: i32,
    url: String,
}

/// ruta para obtener un gif
/// Ejemplo:
/// `127.0.0.1:8000/api/slap/<anime-aqui>/123`
#[get("/api/<action>/<anime>/<gif_id>")]
async fn get_gif(action: String, anime: String, gif_id: i32) -> Option<Json<GifResponse>> {
    let url = format!("localhost:8000/static/{}.gif", gif_id);

    Some(Json(GifResponse {
        action,
        anime,
        gif_id,
        url,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, slap, files, get_gif]).mount("/static", FileServer::from("static"))
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