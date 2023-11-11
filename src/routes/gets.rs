use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};
use rocket::get;
use std::fs;
use rand::Rng;

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
pub async fn random_gif() -> Result<NamedFile, std::io::Error> {
    let path = ".upload/";
    let Ok(file_name) = random_file(path) else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No files in directory"));
    };

    println!("file name: {}", file_name);
    NamedFile::open(Path::new("./upload/").join(file_name)).await
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

/// Uso: `curl -X GET http://localhost:8000/<file_name>`
#[get("/api/<file..>")]
pub async fn files(file: PathBuf) -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("./upload/").join(file)).await
}

#[get("/upload/slap/<file>")]
pub async fn slap(file: PathBuf) -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("./upload/slap/").join(file)).await
}
