mod dbstructs;
mod http_errors;
use actix_files as fs;
use actix_web::{ post, get, web::{self, Json}, App, Error, HttpResponse, Result, HttpServer, web::Data };
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use mongodb::{bson::doc, Client, Database};

const GOOG_BOOK_ROUTE: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn%3D";

async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("pages/index.html")?)
}
async fn scanner() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("pages/scanner.html")?)
}
async fn dashboard() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("pages/dashboard.html")?)
}

#[get("/getUsers")]
async fn get_users(db: Data<Database>,) ->Result<HttpResponse> {
    let mut cursor = db.collection::<dbstructs::User>("users").find(None, None).await.unwrap();
    let mut out: Vec<dbstructs::User> = vec![];
    while cursor.advance().await.unwrap() {
        out.push(cursor.deserialize_current().unwrap());
    }
    Ok(HttpResponse::Ok().json(out))
}

#[post("/addUser")]
async fn add_user(db: Data<Database>, user_json: Json<dbstructs::User>) -> Result<HttpResponse, http_errors::DataError> {
    let user = user_json.into_inner();
    let mut cursor = db.collection::<dbstructs::User>("users").find(None, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        if user.name == cursor.deserialize_current().unwrap().name {
            println!("User {} already added", user.name);
            return Err(http_errors::DataError::DuplicateUser)
        }
    }
    db.collection::<dbstructs::User>("users").insert_one(user.clone(), None).await.unwrap();
    Ok(HttpResponse::Ok().json(user))
}

#[post("/fetchBook")]
async fn fetch_book(obj: Json<dbstructs::BookId>) -> Result<HttpResponse, Error> {
    println!("{}", obj.isbn);
    let book_find: dbstructs::Volumes = reqwest::get(format!("{}{}&maxResults=1", GOOG_BOOK_ROUTE, &obj.isbn)).await.unwrap().json().await.unwrap();
    println!("{}", book_find.items[0].volumeInfo.title);
    Ok(HttpResponse::Ok().json(&book_find.items[0].volumeInfo.into_book(obj.isbn.clone())))
}

#[post("/addBook")]
async fn add_book(db: Data<Database>, book_json: Json<dbstructs::Book>) -> Result<HttpResponse, http_errors::DataError> {
    let book = book_json.into_inner();
    let filter = doc! { "isbn": book.isbn.clone() };
    let mut cursor = db.collection::<dbstructs::Book>("books").find(filter, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        if book.isbn == cursor.deserialize_current().unwrap().isbn {
            println!("{} already added", book.title);
            return Err(http_errors::DataError::DuplicateBook)
        }
    }
    db.collection::<dbstructs::Book>("books").insert_one(book.clone(), None).await.unwrap();
    Ok(HttpResponse::Ok().json(book))
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

    let client = Client::with_uri_str("mongodb://root:rootpassword@localhost:27017").await.unwrap();

    let db_data = Data::new(client.database("testdb"));

    HttpServer::new(move || App::new()
                        .service(fs::Files::new("/static", "./static/").show_files_listing())
                        .app_data(db_data.clone())
                        .route("/", web::get().to(index))
                        .route("/scanner", web::get().to(scanner))
                        .route("/dashboard", web::get().to(dashboard))
                        .service(get_users)
                        .service(fetch_book)
                        .service(add_book)
                        .service(add_user))
        .bind_openssl("0.0.0.0:8100", builder)?
        .run()
        .await
}