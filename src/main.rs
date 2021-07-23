#[macro_use]
extern crate rocket;

mod routes;

use mongodb::{bson::doc, options::ClientOptions, Client};
use dotenv;

#[get("/")] fn check_health() -> &'static str { "OK" }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
  let db_str: String = format!(
    "mongodb://{}:{}@{}:{}",
    dotenv::var("MONGO_USER").unwrap(),
    dotenv::var("MONGO_PASS").unwrap(),
    dotenv::var("MONGO_HOST").unwrap(),
    dotenv::var("MONGO_PORT").unwrap()
  );

  let client_options = ClientOptions::parse(db_str).await.unwrap();
  let client = Client::with_options(client_options).unwrap();

  let db = client.database("local");
  
  rocket::build()
    .manage(db)
    .mount("/health", routes![check_health])
    .mount("/api/v1/books", routes![
        routes::books::get_books, // get
        routes::books::get_book, // get
        routes::books::new_book, // post
        routes::books::update_book, // put
        routes::books::delete_book,// delete
    ]).launch().await
}
