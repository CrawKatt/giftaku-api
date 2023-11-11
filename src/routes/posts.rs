use std::fs;
use std::path::Path;
use rocket::form::Form;
use rocket::response::status::BadRequest;
use uuid::Uuid;
use rocket::{FromForm, post};
use rocket::fs::TempFile;
use rocket::serde::json::{Json, serde_json};

#[derive(FromForm)]
pub struct UploadData<'a> {
    file: TempFile<'a>,
    action: String,
}

/// # Uso:
/// `curl -X POST -F "file=@/path/to/file.gif" -F "action=action" http://localhost:8000/`
/// `path: localhost:8000/file_name`
#[post("/", format = "multipart/form-data", data = "<upload>")]
pub async fn upload(mut upload: Form<UploadData<'_>>) -> Result<Json<String>, BadRequest<&str>> {
    // Verifica que el archivo sea de tipo gif
    if upload.file.content_type() != Some(&rocket::http::ContentType::GIF) {
        return Err(BadRequest("File is not a gif"));
    }

    // Genera el nombre del archivo
    let uuid = Uuid::new_v4();
    let file_name = format!("{uuid}.gif");
    let action = &upload.action;

    // Guarda el archivo en el directorio de archivos upload
    let path = Path::new("./upload").join(action).join(&file_name);

    // Crea el directorio si no existe
    fs::create_dir_all(path.parent().unwrap()).unwrap_or_default();
    upload.file.persist_to(&path).await.unwrap_or_else(|why| {
        eprintln!("Cannot copy file to {}, {why}", path.display());
    });

    // Envía la respuesta con el nombre del archivo en formato json
    let response = format!("http://localhost:8000/api/{file_name}");
    let json = serde_json::json!(response).to_string();
    Ok(Json(json))
}