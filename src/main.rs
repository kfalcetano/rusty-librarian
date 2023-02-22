mod bookschema;
use actix_files as fs;
use actix_web::{error, post, web, App, Error, HttpResponse, Result, HttpServer };
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const GOOG_BOOK_ROUTE: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn%3D";
const MAX_SIZE: usize = 262_144;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    color: String
}

async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("index.html")?)
}

#[post("/fetchBook")]
async fn log_book(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let body = load_body(payload).await.unwrap();
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<bookschema::BookId>(&body)?;
    println!("{}", obj.isbn);
    let book_find: bookschema::Volumes = reqwest::get(format!("{}{}&maxResults=1", GOOG_BOOK_ROUTE, obj.isbn)).await.unwrap().json().await.unwrap();
    println!("{}", book_find.items[0].volumeInfo.title);
    Ok(HttpResponse::Ok().json(&book_find.items[0].volumeInfo))
}

#[post("/addBook")]
async fn add_book(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let body = load_body(payload).await.unwrap();
    // body is loaded, now we can deserialize serde-json
    let info = serde_json::from_slice::<bookschema::VolumeInfo>(&body)?;
    println!("User requested to add {}", info.title);
    Ok(HttpResponse::Ok().json(info))
}

#[post("/addUser")]
async fn add_user(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let body = load_body(payload).await.unwrap();
    // body is loaded, now we can deserialize serde-json
    let user = serde_json::from_slice::<User>(&body)?;
    println!("User {} added with email {} and color {}", user.username, user.email, user.color);
    Ok(HttpResponse::Ok().json(user))
}

async fn load_body(mut payload: web::Payload) -> Result<web::BytesMut, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/C=US/ST=Washington'
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    // wsl2 fuckery:
    // netsh interface portproxy add v4tov4 listenport=8100 listenaddress=0.0.0.0 connectport=8100 connectaddress=<wsl_ip>

    HttpServer::new(|| App::new()
                        .service(fs::Files::new("/static", "./static/").show_files_listing())
                        .route("/", web::get().to(index))
                        .service(log_book)
                        .service(add_book))
        .bind_openssl("0.0.0.0:8100", builder)?
        .run()
        .await
}