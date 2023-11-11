use std::path::Path;
use rocket::response::status::BadRequest;
use uuid::Uuid;
use rocket::post;
use rocket::fs::TempFile;
use rocket::serde::json::{Json, serde_json};

#[post("/", format = "image/gif", data = "<upload>")]
pub async fn upload(mut upload: TempFile<'_>) -> Result<Json<String>, BadRequest<&str>> {
    // Verifica que el archivo sea de tipo gif
    if upload.content_type() != Some(&rocket::http::ContentType::GIF) {
        return Err(BadRequest("File is not a gif"));
    }

    // Genera el nombre del archivo
    let uuid = Uuid::new_v4();
    let file_name = format!("{uuid}.gif");

    // Guarda el archivo en el directorio de archivos upload
    let path = Path::new("./upload").join(&file_name);
    upload.persist_to(&path).await.unwrap_or_else(|why| {
        eprintln!("Cannot copy file to {}, {why}", path.display());
    });

    // Envía la respuesta con el nombre del archivo en formato json
    let response = format!("http://localhost:8000/api/{file_name}");
    let json = serde_json::json!(response).to_string();
    Ok(Json(json))
}

/// # Uso:
/// `curl -X POST -H "Content-Type: text/plain" --data-binary @<file> http://127.0.0.1:8000/`
/// `path: localhost:8000/api/upload/slap`
#[post("/upload/slap", data = "<upload>")]
pub async fn upload_slap(mut upload: TempFile<'_>) -> Result<Json<String>, BadRequest<&str>> {
    // Verifica que el archivo sea de tipo gif
    if upload.content_type() != Some(&rocket::http::ContentType::GIF) {
        return Err(BadRequest("File is not a gif"));
    }

    // Genera el nombre del archivo
    let uuid = Uuid::new_v4();
    let file_name = format!("{}.gif", uuid);

    // Guarda el archivo en el directorio de archivos upload
    let path = Path::new("./upload/slap").join(&file_name);
    upload.persist_to(&path).await.unwrap_or_else(|why| {
        eprintln!("Cannot copy file to {}, {why}", path.display());
    });

    // Envía la respuesta con el nombre del archivo en formato json
    let response = format!("http://localhost:8000/api/{file_name}");
    let json = serde_json::json!(response).to_string();
    Ok(Json(json))
}