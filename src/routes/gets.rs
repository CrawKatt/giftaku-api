use rocket::fs::NamedFile;
use std::path::Path;
use rocket::get;
use std::fs;
use rand::Rng;
use crate::DB;
use crate::routes::posts::SaveData;

/// Buscar el GIF de forma aleatoria en `./upload/`
fn random_file(path: &String) -> std::io::Result<String> {
    let paths: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension() == Some(std::ffi::OsStr::new("gif")))
        .collect();

    if !paths.is_empty() {
        let index = rand::thread_rng().gen_range(0..paths.len());
        return Ok(paths[index].file_name().to_string_lossy().into_owned())
    }

    Err(std::io::Error::new(std::io::ErrorKind::Other, "Error in random_file function: No files in directory"))
}

/// todo: Devolver un JSON para visualizar el GIF en el GET `get_gif` en lugar del GIF directamente
/// todo: Seleccionar el `anime_name` donde el nombre del archivo en la Base de Datos sea igual al nombre del archivo en la URL
#[get("/api/<action>")]
pub async fn send_result(action: &str) -> Result<NamedFile, std::io::Error> {
    let path = format!("./upload/{action}");
    let Ok(file_name) = random_file(&path) else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error in get_gif function: No files in directory"));
    };

    let results: Vec<SaveData> = DB.select("api-database")
        .await
        .unwrap_or_else(|why| {
            panic!("Could not query database: {why}");
        });
    println!("Result: {:#?}", results);

    NamedFile::open(Path::new("./upload/")
        .join(action)
        .join(file_name)).await
}

/// Función para obtener el GIF específico mediante la URL proporcionada por send_result
#[get("/api/<action>/<file_name>")]
pub async fn get_gif(action: &str, file_name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new("./upload/")
        .join(action)
        .join(file_name)).await.ok()
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}