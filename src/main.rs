#[macro_use]
extern crate rocket;

mod routes;

use mongodb::{bson::doc, options::ClientOptions, Client};
use dotenv;

// for CORS settings
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status, Method};
use rocket::yansi::Paint;
use std::io::Cursor;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
  fn info(&self) -> Info {
    Info {
      name: "Add CORS headers to responses",
      kind: Kind::Response,
    }
  }

  async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
    response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE"));
    response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

    // For preflight
    if request.method() == Method::Options {
      info_!("{}", Paint::green("Maybe preflight?"));
      let rv = "";
      response.set_status(Status::Ok);
      response.set_sized_body(rv.len(), Cursor::new(rv));
    }
  }
}

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
    .attach(CORS)
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
