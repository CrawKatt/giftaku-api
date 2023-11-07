use std::fs;
use rocket::serde::json::{Json, serde_json};
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use rand::Rng;
use rocket::{get, launch, post, routes};
use rocket::fs::{FileServer, TempFile};
use rocket::fs::NamedFile;
use rocket::response::status::BadRequest;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use uuid::Uuid;

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

/// Buscar el GIF de forma aleatoria en `./upload/`
fn random_file(path: &str) -> std::io::Result<String> {
    let paths: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension() == Some(std::ffi::OsStr::new("gif")))
        .collect();

    if !paths.is_empty() {
        let index = rand::thread_rng().gen_range(0..paths.len());
        return Ok(paths[index].file_name().to_string_lossy().into_owned())
    }

    Err(std::io::Error::new(std::io::ErrorKind::Other, "No files in directory"))
}

#[get("/api/random")]
async fn random_gif() -> Result<NamedFile, std::io::Error> {
    let path = ".upload/";
    let Ok(file_name) = random_file(path) else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No files in directory"));
    };

    println!("file name: {}", file_name);
    NamedFile::open(Path::new("./upload/").join(file_name)).await
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

/// Uso: `curl -X GET http://localhost:8000/<file_name>`
#[get("/api/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("./upload/").join(file)).await
}

#[post("/", format = "image/gif", data = "<upload>")]
async fn upload(mut upload: TempFile<'_>) -> Result<Json<String>, BadRequest<&str>> {
    // Verifica que el archivo sea de tipo gif
    if upload.content_type() != Some(&rocket::http::ContentType::GIF) {
        return Err(BadRequest("File is not a gif"));
    }

    // Genera el nombre del archivo
    let uuid = Uuid::new_v4();
    let file_name = format!("{uuid}.gif");

    // Guarda el archivo en el directorio de archivos upload
    let path = Path::new("./upload").join(&file_name);
    upload.copy_to(&path).await.unwrap_or_else(|why| {
        eprintln!("Cannot copy file to {}, {why}", path.display());
    });

    // Envía la respuesta con el nombre del archivo en formato json
    let response = format!("http://localhost:8000/api/{file_name}");
    let json = serde_json::json!(response).to_string();
    Ok(Json(json))
}

#[get("/upload/slap/<file>")]
async fn slap(file: PathBuf) -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("./upload/slap/").join(file)).await
}

/// # Uso:
/// `curl -X POST -H "Content-Type: text/plain" --data-binary @<file> http://127.0.0.1:8000/`
/// `path: localhost:8000/api/upload/slap`
#[post("/upload/slap", data = "<upload>")]
async fn upload_slap(mut upload: TempFile<'_>) -> Result<Json<String>, BadRequest<&str>> {
    // Verifica que el archivo sea de tipo gif
    if upload.content_type() != Some(&rocket::http::ContentType::GIF) {
        return Err(BadRequest("File is not a gif"));
    }

    // Genera el nombre del archivo
    let uuid = Uuid::new_v4();
    let file_name = format!("{}.gif", uuid);

    // Guarda el archivo en el directorio de archivos upload
    let path = Path::new("./upload/slap").join(&file_name);
    upload.copy_to(&path).await.unwrap_or_else(|why| {
        eprintln!("Cannot copy file to {}, {why}", path.display());
    });

    // Envía la respuesta con el nombre del archivo en formato json
    let response = format!("http://localhost:8000/api/{file_name}");
    let json = serde_json::json!(response).to_string();
    Ok(Json(json))
}