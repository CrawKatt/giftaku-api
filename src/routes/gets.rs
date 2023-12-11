use rocket::fs::NamedFile;
use std::path::Path;
use rocket::get;
use std::fs;
use rand::Rng;
use rand::seq::SliceRandom;
use rocket::serde::json::Json;
use serde::Serialize;
use crate::{DB, URL_HOST};
use crate::routes::posts::SaveData;

const ENDPOINTS: [&str; 5] = ["slap", "shoot", "kick", "punch", "cringe"];

#[derive(Serialize)]
pub struct ResponseData {
    pub anime_name: String,
    pub url: String,
}

impl ResponseData {
    const fn new(anime_name: String, url: String) -> Self {
        Self {
            anime_name,
            url,
        }
    }
}

#[derive(Serialize)]
pub struct ResponseWrapper {
    results: Vec<ResponseData>,
}

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

#[get("/api/<action>")]
pub async fn send_result(action: &str) -> Result<Json<ResponseData>, std::io::Error> {
    DB.use_ns("api-namespace").use_db("api-db").await.ok();

    let path = format!("./upload/{action}");
    let file_name = random_file(&path)?;

    // NO USAR "-" COMO REEMPLAZO A LOS ESPACIOS AL LLAMAR A LA TABLA EN EL FROM, SURREALDB LO INTERPRETA COMO UNA OPERACIÓN
    // SI OCURREN ERRORES, DEBUGEAR LA QUERY TOKENIZADA QUITANDO EL TIPO EN QUERY_RESULT Y REMOVIENDO EL AWAIT Y EL TAKE
    let sql_query = "SELECT * FROM api_uploads WHERE file_name = $file_name";
    let query_result: Vec<SaveData> = DB.query(sql_query)
        .bind(("file_name", &file_name)).await.unwrap_or_else(|why| {
            panic!("Could not query database: {why}");
        }).take(0).unwrap_or_else(|why| {
            eprintln!("Error in send_result function: No results in query: {why}");
            vec![]
        });
    println!("Query result: {query_result:#?}");

    if query_result.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error in GET: No results in query"));
    }

    let anime_name = &query_result[0].anime_name;
    let anime_name = anime_name.to_owned();
    let response_data = ResponseData::new(anime_name, format!("https://{}/api/{action}/{file_name}", *URL_HOST));

    Ok(Json(response_data))
}

/// Función para obtener el GIF específico mediante la URL proporcionada por send_result
#[get("/api/<action>/<file_name>")]
pub async fn get_gif(action: &str, file_name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new("./upload/")
        .join(action)
        .join(file_name)).await.ok()
}

#[get("/api/<action>?<amount>")]
pub async fn get_gif_amount(action: &str, amount: usize) -> Result<Json<ResponseWrapper>, std::io::Error> {
    DB.use_ns("api-namespace").use_db("api-db").await.ok();

    let sql_query = "SELECT * FROM api_uploads";
    let query_result: Vec<SaveData> = DB.query(sql_query).await.unwrap_or_else(|why| {
        panic!("Could not query database: {why}");
    }).take(0).unwrap_or_else(|why| {
        eprintln!("Error in send_result function: No results in query: {why}");
        vec![]
    });

    if query_result.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error in GET: No results in query"));
    }

    let mut rng = rand::thread_rng();
    let random_results: Vec<_> = query_result.choose_multiple(&mut rng, amount).collect();

    let response_data: Vec<_> = random_results.into_iter().map(|result| {
        let anime_name = result.anime_name.clone();
        let file_name = result.file_name.clone();
        ResponseData::new(anime_name, format!("https://{}/api/{action}/{file_name}", *URL_HOST))
    }).collect();

    Ok(Json(ResponseWrapper { results: response_data } ))
}

#[get("/api/endpoints")]
pub async fn get_endpoints() -> String {
    let mut json_endpoints = String::from("{");

    // Agrega los endpoints predeterminados
    for (index, endpoint) in ENDPOINTS.iter().enumerate() {
        json_endpoints.push_str(format!(r#""{endpoint}":{{"format":"gif"}}"#).as_str());

        // Agrega una coma si no es el último elemento
        if index < ENDPOINTS.len() - 1 {
            json_endpoints.push(',');
        }
    }

    json_endpoints.push('}');

    json_endpoints
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}