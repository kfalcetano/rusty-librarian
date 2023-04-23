mod dbstructs;
mod http_errors;

use actix_files as fs;
use actix_web::{ post, get, web::{self, Json}, App, Error, HttpResponse, Result, HttpServer, web::Data};
use dbstructs::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use mongodb::{bson, bson::{doc, Document}, Client, Database, options::FindOptions, Collection};
use serde_json::json;

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
async fn search() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("pages/search.html")?)
}

async fn find_in_collection<T: serde::de::DeserializeOwned>(collection: Collection<T>, filter: Option<Document>, options: Option<FindOptions>) -> Result<Vec<T>, mongodb::error::Error> {
    let mut cursor= collection.find(filter, options).await?;
    let mut out: Vec<T> = vec![];
    while cursor.advance().await.unwrap() {
        out.push(cursor.deserialize_current()?);
    }
    Ok(out)
}

#[get("/api/getUsers")]
async fn get_users(db: Data<Database>,) ->Result<HttpResponse> {
    let out = find_in_collection(db.collection::<User>("users"), None, None).await.unwrap();
    Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(out))
}

#[get("/api/getBookList")]
async fn get_book_list(db: Data<Database>, query: web::Query<BookListQuery>) ->Result<HttpResponse> {
    let filter: Option<bson::Document>;
    match (query.clone().into_inner().username, query.clone().into_inner().filter) {
        (Some(username), Some(BookListFilter::Read)) => filter = Some(doc! {"ratings": { "$elemMatch": { "username": username}}}),
        (Some(username), Some(BookListFilter::Unread)) => filter = Some(doc! {"ratings": { "$not": {"$elemMatch": { "username": username}}}}),
        _ => filter = None
    }
    let direction: i32;
    match query.clone().into_inner().direction {
        Some(SortDirection::Up) => direction = -1,
        Some(SortDirection::Down) => direction = 1,
        None => direction = 1
    }
    let sort: bson::Document;
    match query.into_inner().sort {
        Some(BookListSort::Author) => sort = doc! {"authors.0": direction},
        Some(BookListSort::Date) => sort = doc! {"publishedDate": direction},
        Some(BookListSort::Genre) => sort = doc! {"categories.0": direction, "title": 1},
        Some(BookListSort::Title) => sort = doc! {"title": direction},
        None => sort = doc! {"title": 1}
    }
    let find_options = FindOptions::builder().sort(sort).build();
    let out=find_in_collection(db.collection::<BookListElement>("books"), filter, Some(find_options)).await.unwrap();
    Ok(HttpResponse::Ok().json(out))
}

#[get("/book/{isbn}")]
async fn book_page(db: Data<Database>, isbn: web::Path<String>) -> Result<fs::NamedFile, http_errors::DataError> {
    let id = isbn.into_inner();
    let filter = doc! { "isbn": &id };
    let mut cursor = db.collection::<BookListElement>("books").find(filter, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        let book = cursor.deserialize_current().unwrap();
        if id == book.isbn {
            return Ok(fs::NamedFile::open("pages/bookpage.html").unwrap())
        }
    }
    return Err(http_errors::DataError::BookNotFound)
}

#[get("/api/book/{isbn}")]
async fn get_book(db: Data<Database>, isbn: web::Path<String>) -> Result<HttpResponse> {
    let id = isbn.into_inner();
    let filter = doc! { "isbn": &id };
    let mut cursor = db.collection::<Book>("books").find(filter, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        let book = cursor.deserialize_current().unwrap();
        if id == book.isbn {
            return Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(book))
        }
    }
    return Ok(HttpResponse::NotFound().json(format!("Book with isbn {} not found", id)))
}

#[post("/api/addUser")]
async fn add_user(db: Data<Database>, user_json: Json<User>) -> Result<HttpResponse, http_errors::DataError> {
    let user = user_json.into_inner();
    let mut cursor = db.collection::<User>("users").find(None, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        if user.name == cursor.deserialize_current().unwrap().name {
            return Err(http_errors::DataError::DuplicateUser)
        }
    }
    db.collection::<User>("users").insert_one(user.clone(), None).await.unwrap();
    Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(user))
}

#[post("/api/fetchBook")]
async fn fetch_book(db: Data<Database>, obj: Json<BookId>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let filter = doc! { "isbn": obj.isbn.clone() };
    let mut cursor = db.collection::<Book>("books").find(filter, None).await?;
    while cursor.advance().await? {
        let book = cursor.deserialize_current()?;
        if obj.isbn == book.isbn {
            return Ok(HttpResponse::Found().append_header(("Cache-Control", "no-cache")).json(book))
        }
    }
    let book_find: Volumes = reqwest::get(format!("{}{}&maxResults=1", GOOG_BOOK_ROUTE, &obj.isbn)).await?.json().await?;
    println!("{}", book_find.items[0].volumeInfo.title);
    Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(&book_find.items[0].volumeInfo.into_book(obj.isbn.clone())))
}

#[post("/api/addBook")]
async fn add_book(db: Data<Database>, book_json: Json<Book>) -> Result<HttpResponse, http_errors::DataError> {
    let book = book_json.into_inner();
    let filter = doc! { "isbn": book.isbn.clone() };
    let mut cursor = db.collection::<Book>("books").find(filter, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        if book.isbn == cursor.deserialize_current().unwrap().isbn {
            println!("{} already added", book.title);
            return Err(http_errors::DataError::DuplicateBook)
        }
    }
    db.collection::<Book>("books").insert_one(book.clone(), None).await.unwrap();
    Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(book))
}

#[post("/api/deleteBook")]
async fn delete_book(db: Data<Database>, obj: Json<BookId>) -> Result<HttpResponse, Error> {
    let dr = db.collection::<Book>("books").delete_one(doc!{"isbn": obj.isbn.clone()}, None).await.unwrap();
    Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(json!({"num_deleted": dr.deleted_count})))
}

#[post("/api/deleteUser")]
async fn delete_user(db: Data<Database>, obj: Json<User>) -> Result<HttpResponse, Error> {
    let dr = db.collection::<Book>("users").delete_one(doc!{"name": obj.name.clone()}, None).await.unwrap();
    Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(json!({"num_deleted": dr.deleted_count})))
}

#[post("/api/addComment/{isbn}")]
async fn add_comment(db: Data<Database>, isbn: web::Path<String>, comment: Json<Comment>) -> Result<HttpResponse, Error> {
    let filter = doc!{"isbn": isbn.into_inner()};
    let update = doc!{"$push": {"comments": bson::to_bson(&comment.into_inner()).unwrap()}};
    db.collection::<Book>("books").update_one(filter, update, None).await.unwrap();
    Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(json!({"message": "added comment"})))
}

#[post("/api/rateBook/{isbn}")]
async fn rate_book(db: Data<Database>, isbn: web::Path<String>, rating: Json<Rating>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let coll = db.collection::<Book>("books");
    let book = coll.find_one(doc!{"isbn": isbn.clone()}, None).await?;
    if book.is_none() {
        return Err(Box::new(http_errors::DataError::BookNotFound))
    }
    if db.collection::<User>("users").find_one(doc!{"name": rating.username.clone()}, None).await?.is_none() {
        return Err(Box::new(http_errors::DataError::UserNotFound))
    }
    match book.unwrap().ratings.iter().find(|x| x.username == rating.username) {
        Some(_) => {
            let filter = doc!{"isbn": isbn.clone(), "ratings.username": rating.username.clone()};
            let edit_update = doc!{"$set": {"ratings.$": bson::to_bson(&rating.into_inner())?}};
            coll.update_one(filter, edit_update, None).await?;
            Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(json!({"message": "updated rating"})))
        }
        None => {
            let push_update = doc!{"$push": {"ratings": bson::to_bson(&rating.into_inner())?}};
            coll.update_one(doc!{"isbn": isbn.into_inner()}, push_update, None).await?;
            Ok(HttpResponse::Ok().append_header(("Cache-Control", "no-cache")).json(json!({"message": "added rating"})))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // mkdir -p certs && openssl req -x509 -newkey rsa:4096 -nodes -keyout certs/key.pem -out certs/cert.pem -days 365 -subj '/C=US/ST=Washington'
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("certs/key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("certs/cert.pem").unwrap();

    // wsl2 fuckery:
    // netsh interface portproxy add v4tov4 listenport=8100 listenaddress=0.0.0.0 connectport=8100 connectaddress=<wsl_ip>

    let client = Client::with_uri_str("mongodb://datastore:27017").await.unwrap();

    let db_data = Data::new(client.database("testdb"));

    HttpServer::new(move || App::new()
                        .service(fs::Files::new("/static", "./static/").show_files_listing())
                        .app_data(db_data.clone())
                        .route("/", web::get().to(index))
                        .route("/scanner", web::get().to(scanner))
                        .route("/dashboard", web::get().to(dashboard))
                        .route("/search", web::get().to(search))
                        .service(get_users)
                        .service(fetch_book)
                        .service(add_book)
                        .service(add_user)
                        .service(get_book_list)
                        .service(book_page)
                        .service(get_book)
                        .service(delete_book)
                        .service(delete_user)
                        .service(add_comment)
                        .service(rate_book))
        .bind_openssl("0.0.0.0:8100", builder)?
        .run()
        .await
}