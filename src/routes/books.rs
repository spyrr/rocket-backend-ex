use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Database};
use nanoid::nanoid;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::State;
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
  id: Option<String>,
  title: String,
  author: String
}

#[get("/")]
pub async fn get_books(db: &State<Database>) -> String {
  let collection = db.collection::<Book>("books");
  let mut books = collection.find(None, None).await.unwrap();
  let mut res: Vec<Book> = Vec::new();

  while let Some(doc) = books.try_next().await.unwrap() {
    res.push(doc);
  }
  
  json!(res).to_string()
}

#[get("/<id>")]
pub async fn get_book(db: &State<Database>, id: &str) -> String {
  let filter = doc! { "id": id };
  let collection = db.collection::<Book>("books");
  let books = collection.find_one(filter, None).await.unwrap();

  json!(books).to_string()
}

#[post("/", data="<body>")]
pub async fn new_book(db: &State<Database>, body: Json<Book>) -> String {
  let Json(new) = body;
  let id = nanoid!(8);

  let book = Book {
    id: Some(id.clone()),
    title: new.title,
    author: new.author
  };
  
  let collection = db.collection("books");
  let _rv = collection.insert_one(book, None).await;
  let book = collection.find_one(doc! { "id": id}, None).await.unwrap();

  json!(book).to_string()
}

#[put("/<id>", data="<body>")]
pub async fn update_book(db: &State<Database>, id: &str, body: Json<Book>) -> String {
  let Json(book) = body;
  let collection = db.collection::<Book>("books");
  let _rv = collection.update_one(
    doc! { "id": id },
    doc! {"$set": {"title": book.title, "author": book.author}},
    None
  ).await;

  let book = collection.find_one(doc! { "id": id }, None).await.unwrap();
  json!(book).to_string()
}

#[delete("/<id>")]
pub async fn delete_book(db: &State<Database>, id: &str) -> &'static str {
  let filter = doc! { "id": id };
  let collection = db.collection::<Book>("books");
  let _rv = collection.delete_one(filter, None).await;
  "Removed"
}
