use std::fs;
use std::path::Path;
use rocket::form::Form;
use rocket::response::status::BadRequest;
use uuid::Uuid;
use rocket::{FromForm, post};
use rocket::fs::TempFile;
use rocket::serde::json::{Json, serde_json};
use serde::{Deserialize, Serialize};
use surrealdb::Result as SurrealResult;
use crate::{DB, RocketResult, URL_HOST};

#[derive(FromForm)]
pub struct UploadData<'a> {
    file: TempFile<'a>,
    action: &'a str,
    anime_name: &'a str,
}

#[derive(Serialize)]
struct GifData<'a> {
    anime_name: &'a str,
    url: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveData {
    pub file_name: String,
    pub anime_name: String,
    pub url: String,
}

impl SaveData {
    const fn new(file_name: String, anime_name: String, url: String) -> Self {
        Self {
            file_name,
            anime_name,
            url,
        }
    }

    // This function saves the data to the database
    async fn save_data_to_db(&self) -> SurrealResult<()> {
        DB.use_ns("api-namespace").use_db("api-db").await?;
        let data = Self::new(self.file_name.clone(), self.anime_name.clone(), self.url.clone());
        let created: Vec<Self> = DB.create("api_uploads").content(data).await?; // NO USAR "-" COMO REEMPLAZO A LOS ESPACIOS AL CREAR EL RESOURCE, SURREALDB LO INTERPRETA COMO UNA OPERACIÓN
        println!("Created: {created:#?}");
        Ok(())
    }
}

impl <'a>GifData<'a> {
    const fn new(anime_name: &'a str, url: &'a str) -> Self {
        Self {
            anime_name,
            url,
        }
    }
}

impl UploadData<'_> {
    fn check_type(&self) -> RocketResult<()> {
        if self.file.content_type() == Some(&rocket::http::ContentType::GIF) {
            Ok(())
        } else {
            Err(BadRequest(String::from("File is not a gif")))
        }
    }

    async fn save_file(&mut self) -> RocketResult<String> {
        let uuid = Uuid::new_v4();
        let file_name = format!("{uuid}.gif");
        let action = &self.action;
        let path = Path::new("./upload").join(action).join(&file_name);
        fs::create_dir_all(path.parent().unwrap()).unwrap_or_default();
        self.file.persist_to(&path).await.unwrap_or_else(|why| {
            eprintln!("Cannot copy file to {}, {why}", path.display());
        });
        Ok(file_name)
    }

    fn get_url(&self, file_name: &String) -> String {
        format!("http://{}:8000/api/{}/{}", *URL_HOST, self.action, file_name)
    }
}

/// # Uso:
/// `curl -X POST -F "file=@/path/to/file.gif" -F "action=action" http://0.0.0.0:8000/`
/// `path: 0.0.0.0:8000/file_name`
#[post("/", format = "multipart/form-data", data = "<upload>")]
pub async fn upload(mut upload: Form<UploadData<'_>>) -> RocketResult<Json<String>> {

    // Verifica que el archivo sea un GIF
    upload.check_type()?;

    // Guarda el GIF y devuelve el nombre del archivo
    let response = &upload.save_file().await?;

    // Crea un objeto GifData y lo convierte a json
    let data = GifData::new(upload.anime_name, response);

    let database_data = SaveData::new(response.to_owned(), upload.anime_name.to_string(), upload.get_url(response));
    database_data.save_data_to_db().await.unwrap_or_else(|why| {
        eprintln!("Cannot save data to database: {why}");
    });

    // Envía la respuesta con el nombre del archivo en formato json
    let json = serde_json::json!(data).to_string();
    Ok(Json(json))
}