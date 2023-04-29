mod api;
mod models;
mod repository;
mod test;

#[macro_use]
extern crate rocket;

use api::recipe_api::{create_recipe, delete_recipe, get_all_recipes, get_recipe, update_recipe};
use repository::mongodb_recipe::MongoRepo;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[launch]
pub fn rocket() -> _ {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![create_recipe])
        .mount("/", routes![get_recipe])
        .mount("/", routes![update_recipe])
        .mount("/", routes![delete_recipe])
        .mount("/", routes![get_all_recipes])
        .attach(cors)
}
