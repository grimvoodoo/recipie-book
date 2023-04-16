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

// #[cfg(test)]
// mod test {
//     use super::*;
//     use rocket::http::Status;
//     use rocket::local::blocking::Client;

//     #[test]
//     fn test_get_all_users() {
//         let client = Client::tracked(rocket()).unwrap();
//         let response = client.get("/users").dispatch();
//         assert_eq!(response.status(), Status::Ok);
//     }

//     #[test]
//     fn test_create_user() {
//         let client = Client::tracked(rocket()).unwrap();
//         let response = client
//             .post("/user")
//             .body(r#"{"name": "test", "location": "test", "title": "test"}"#)
//             .dispatch();
//         assert_eq!(response.status(), Status::Ok);
//     }

//     #[test]
//     fn test_get_user() {
//         let client = Client::tracked(rocket()).unwrap();
//         let response = client.get("/user/test").dispatch();
//         assert_eq!(response.status(), Status::Ok);
//     }

//     // #[test]
//     // fn test_update_user() {
//     //     let client = Client::tracked(rocket()).unwrap();
//     //     let response = client
//     //         .put("/user/test")
//     //         .body(r#"{"name": "Mr test", "location": "testville", "title": "tester"}"#)
//     //         .dispatch();
//     //     assert_eq!(response.status(), Status::Ok);
//     // }

//     // #[test]
//     // fn test_delete_user() {
//     //     let client = Client::tracked(rocket()).unwrap();
//     //     let response = client.delete("/user/test").dispatch();
//     //     assert_eq!(response.status(), Status::Ok);
//     // }
// }
