use rocket::tokio::fs::File;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use mongodb::bson::doc;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rocket::{get, launch, post, routes};
use rocket::fs::FileServer;
use rocket::http::Status;
use tokio::io::AsyncReadExt;
use rocket::data::ByteUnit;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

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
#[tokio::main]
async fn rocket() -> _ {

    DB.connect::<Mem>(()).await.unwrap_or_else(|why| {
        panic!("Could not connect to database: {}", why);
    });

    rocket::build()
        .mount("/", routes![files, get_gif, upload, get_file])
        .mount("/static", FileServer::from("static"))
}

#[get("/<file>")]
async fn files(file: PathBuf) -> Option<Json<String>> {
    let mut buffer = Vec::new();
    let mut file = File::open(Path::new("static/").join(file)).await.ok()?;
    let file_name = file.read_to_end(&mut buffer).await.ok()?;
    let url = format!("data:image/gif/{}", file_name);
    Some(Json(url))
}

/// # Uso:
/// `curl -F "upload=@<file>" localhost:8000/api/upload`
#[post("/api/upload", format = "multipart/form-data", data = "<upload>")]
async fn upload(upload: rocket::Data<'_>) -> Result<Json<String>, Status> {
    // genera el id aleatorio
    let mut file;
    let mut rng = StdRng::from_entropy();
    let random_string: u64 = rng.gen();
    let file_name = format!("{}", random_string);

    // crea el archivo
    if let Err(_) = File::create(&file_name).await {
        return Err(Status::InternalServerError);
    } else {
        file = File::create(&file_name).await.unwrap();
    }

    if let Err(_) = upload.open(ByteUnit::default()).stream_to(&mut file).await {
        return Err(Status::InternalServerError);
    }

    DB.use_ns("api-namespace").use_db("api-db").await.unwrap_or_else(|why| {
        println!("Could not connect to database: {}", why);
    });

    let data = file_name.as_bytes().to_vec();

    DB.set("api-collection", data.clone()).await.unwrap_or_else(|why| {
        println!("Could not connect to database: {}", why);
    });

    println!("File uploaded: {:?}", data);
    println!("File name: {:?}", file_name);

    //database(&file_name).await.expect("ERROR");

    Ok(Json(format!("{{'url': '{}'}}", &file_name)))
}

#[get("/api/<file>")]
async fn get_file(file: String) -> Option<Json<String>> {
    let mut buffer = Vec::new();
    let mut file = File::open(Path::new("./").join(file)).await.ok()?;
    let file_name = file.read_to_end(&mut buffer).await.ok()?;
    let url = format!("data:image/gif/{}", file_name);
    Some(Json(url))
}