mod api;
mod models;
mod repository;
mod test;

#[macro_use]
extern crate rocket;

use api::recipe_api::{create_recipe, delete_recipe, get_all_recipes, get_recipe, update_recipe};
use repository::mongodb_recipe::MongoRepo;

#[launch]
pub fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_recipe])
        .mount("/", routes![get_recipe])
        .mount("/", routes![update_recipe])
        .mount("/", routes![delete_recipe])
        .mount("/", routes![get_all_recipes])
}
