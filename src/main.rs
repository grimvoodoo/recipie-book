mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

#[launch]
pub fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    #[rocket::async_test]
    async fn test_get_all_users() {
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client.get("/users").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_create_user() {
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client
            .post("/user")
            .body(r#"{"id": "5f9f1b5b9c9d2b0b8c1c1c1c", "name": "test", "location": "test", "title": "test"}"#)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_get_user() {
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client
            .get("/user/5f9f1b5b9c9d2b0b8c1c1c1c")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_update_user() {
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client
            .put("/user/5f9f1b5b9c9d2b0b8c1c1c1c")
            .body(r#"{"name": "Mr test", "location": "testville", "title": "tester"}"#)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_delete_user() {
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client
            .delete("/user/5f9f1b5b9c9d2b0b8c1c1c1c")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
    }
}
