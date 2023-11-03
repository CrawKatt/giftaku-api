use rocket::serde::json::{Json, serde_json};
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rocket::{get, launch, post, routes};
use rocket::fs::{FileServer, TempFile};
use rocket::fs::NamedFile;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use std::env;

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[launch]
async fn rocket() -> _ {
    DB.connect::<Mem>(()).await.unwrap_or_else(|why| {
        panic!("Could not connect to database: {}", why);
    });

    rocket::build()
        .mount("/", routes![upload, files, index])
        .mount("/static", FileServer::from("static"))
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

/// Uso: `curl -X GET http://localhost:8000/<file_name>`
#[get("/api/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("/tmp/").join(file)).await
}

/// # Uso:
/// `curl -X POST -H "Content-Type: text/plain" --data-binary @<file> http://127.0.0.1:8000/`
/// `path: localhost:8000/api/upload`
/// todo: cambiar la ruta a ./static/
#[post("/", data = "<upload>")]
async fn upload(mut upload: TempFile<'_>) -> Result<Json<String>, ()> {
    // Genera el nombre del archivo
    let mut rng = StdRng::from_entropy();
    let random_string: u64 = rng.gen();
    let file_name = format!("{}", random_string);

    // Guarda el archivo en el directorio temporal
    let temp = env::temp_dir();
    let path = Path::new(&temp).join(&file_name);
    let Ok(_) = upload.persist_to(&path).await else {
        eprintln!("Cannot create temp file in {}", path.display());
        return Err(());
    };

    // Envía la respuesta con el nombre del archivo en formato json
    let json = serde_json::json!({"url: http://localhost:8000/":file_name}).to_string();
    Ok(Json(json))
}