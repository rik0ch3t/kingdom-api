  
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
pub struct Panthera {
  pub id: Option<i32>,
  pub common_name: String,
  pub scientific_name: String,
  pub species: String,
  pub sub_species: String,
  pub roar: bool
}

#[post("/", data = "<panthera>")]
fn create(panthera: Json<Panthera>) -> Json<Panthera> {
    panthera
}

#[get("/")]
fn read() -> Json<JsonValue> {
    Json(json!([
        "cat 1",
        "cat 2"
    ]))
}

#[put("/<id>", data = "<panthera>")]
fn update(id: i32, panthera: Json<Panthera>) -> Json<Panthera> {
    panthera
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<JsonValue> {
    Json(json!({"status": "ok"}))
}

fn main() {
    rocket::ignite()
        .mount("/panthera", routes![create])
        .mount("/pantheras", routes![read])
        .launch();
}
