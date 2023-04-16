use std::{any::Any, env};
extern crate dotenv;
use crate::models::user_model::User;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};
use std::sync::Mutex;

pub trait UserRepository {
    fn create_user(&self, new_user: User) -> Result<CustomInsertOneResult, RepoError>;
    fn get_user(&self, id: &String) -> Result<User, RepoError>;
    fn update_user(&self, id: &String, new_user: User) -> Result<Box<dyn Any>, RepoError>;
    fn delete_user(&self, id: &String) -> Result<Box<dyn Any>, RepoError>;
    fn get_all_users(&self) -> Result<Vec<User>, RepoError>;
}

pub enum CustomInsertOneResult {
    Real(InsertOneResult),
    Mock(ObjectId),
}
#[derive(Debug)]
pub enum RepoError {
    Mongo(mongodb::error::Error),
    Custom(String),
}

pub struct MongoRepo {
    col: Collection<User>,
}

pub struct MockRepo {
    users: Mutex<Vec<User>>,
}

pub struct MockUpdateResult {
    pub modified_count: i64,
    pub matched_count: i64,
    pub upserted_id: Option<ObjectId>,
}

pub struct MockDeleteResult {
    pub deleted_count: u64,
}

impl From<mongodb::error::Error> for RepoError {
    fn from(err: mongodb::error::Error) -> Self {
        RepoError::Mongo(err)
    }
}

impl From<InsertOneResult> for CustomInsertOneResult {
    fn from(result: InsertOneResult) -> Self {
        CustomInsertOneResult::Real(result)
    }
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    fn get_user_by_name(&self, name: &String) -> Result<User, RepoError> {
        let filter = doc! {"name": name};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        user_detail.ok_or(RepoError::Custom("User not found".to_string()))
    }

    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}

impl MockRepo {
    pub fn new() -> Self {
        MockRepo {
            users: Mutex::new(Vec::new()),
        }
    }
}

impl UserRepository for MongoRepo {
    fn create_user(&self, new_user: User) -> Result<CustomInsertOneResult, RepoError> {
        let new_doc = User {
            id: new_user.id,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user.into())
    }

    // fn get_user(&self, id: &String) -> Result<User, RepoError> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let user_detail = self
    //         .col
    //         .find_one(filter, None)
    //         .ok()
    //         .expect("Error getting user's detail");
    //     Ok(user_detail.unwrap())
    // }

    fn get_user(&self, name: &String) -> Result<User, RepoError> {
        let filter = doc! {"name": name};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        user_detail.ok_or(RepoError::Custom(String::from("User not found")))
    }

    fn update_user(
        &self,
        id: &String,
        new_user: User,
    ) -> Result<Box<(dyn std::any::Any + 'static)>, RepoError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(Box::new(updated_doc))
    }

    fn delete_user(&self, id: &String) -> Result<Box<(dyn Any)>, RepoError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(Box::new(user_detail))
    }

    fn get_all_users(&self) -> Result<Vec<User>, RepoError> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}

impl UserRepository for MockRepo {
    fn create_user(&self, new_user: User) -> Result<CustomInsertOneResult, RepoError> {
        // Use a Mutex to ensure safe concurrent access to the users vector.
        let mut users = self.users.lock().unwrap();

        let id = {
            if new_user.id == None {
                None
            } else {
                new_user.id
            }
        };

        let new_doc = User {
            id,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        users.push(new_doc);

        // Return a dummy InsertOneResult as we don't use MongoDB in this mock implementation.
        Ok(CustomInsertOneResult::Mock(ObjectId::new()))
    }

    fn get_user(&self, id: &String) -> Result<User, RepoError> {
        let users = self.users.lock().unwrap();
        users
            .iter()
            .find(|u| u.id.as_ref().map_or(false, |uid| uid.to_string() == *id))
            .cloned()
            .ok_or(RepoError::Custom("User not found".to_string()))
    }

    fn update_user(&self, id: &String, new_user: User) -> Result<Box<dyn Any>, RepoError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users
            .iter_mut()
            .find(|u| u.id.as_ref().map_or(false, |uid| uid.to_string() == *id))
        {
            *user = new_user;

            // Return a dummy UpdateResult as we don't use MongoDB in this mock implementation.
            Ok(Box::new(MockUpdateResult {
                matched_count: 1,
                modified_count: 1,
                upserted_id: None,
            }))
        } else {
            Err(RepoError::Custom("User not found".to_string()))
        }
    }

    fn delete_user(&self, id: &String) -> Result<Box<(dyn Any)>, RepoError> {
        let mut users = self.users.lock().unwrap();
        let initial_len = users.len();
        users.retain(|u| u.id.as_ref().map_or(true, |uid| uid.to_string() != *id));

        // Return a dummy DeleteResult as we don't use MongoDB in this mock implementation.
        Ok(Box::new(MockDeleteResult {
            deleted_count: (initial_len - users.len()) as u64,
        }))
    }

    fn get_all_users(&self) -> Result<Vec<User>, RepoError> {
        let users = self.users.lock().unwrap();
        Ok(users.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::oid::ObjectId;
    use std::sync::Arc;

    fn test_user() -> User {
        User {
            id: Some(ObjectId::new()),
            name: "John Doe".to_string(),
            location: "New York".to_string(),
            title: "Software Developer".to_string(),
        }
    }

    #[test]
    fn test_create_user() {
        let mock_repo = Arc::new(MockRepo::new());
        let test_user = test_user();

        let result = mock_repo.create_user(test_user).unwrap();

        if let CustomInsertOneResult::Mock(_) = result {
            // Test passed, as the result is the Mock variant
        } else {
            panic!("Expected Mock variant");
        }
    }

    #[test]
    fn test_get_user() {
        let mock_repo = Arc::new(MockRepo::new());
        let test_user = test_user();
        let user_id = test_user.id.clone().unwrap().to_string();

        mock_repo.users.lock().unwrap().push(test_user);

        let result = mock_repo.get_user(&user_id).unwrap();

        assert_eq!(result.id.unwrap().to_string(), user_id);
    }

    #[test]
    fn test_update_user() {
        let mock_repo = Arc::new(MockRepo::new());
        let mut test_user = test_user();
        let user_id = test_user.id.clone().unwrap().to_string();

        mock_repo.users.lock().unwrap().push(test_user.clone());

        test_user.title = "Senior Software Developer".to_string();
        let result = mock_repo.update_user(&user_id, test_user).unwrap();
        let result = result.downcast_ref::<MockUpdateResult>().unwrap();

        assert_eq!(result.matched_count, 1);
        assert_eq!(result.modified_count, 1);
    }

    #[test]
    fn test_delete_user() {
        let mock_repo = Arc::new(MockRepo::new());
        let test_user = test_user();
        let user_id = test_user.id.clone().unwrap().to_string();

        mock_repo.users.lock().unwrap().push(test_user);

        let result = mock_repo.delete_user(&user_id).unwrap();
        let result = result.downcast_ref::<MockDeleteResult>().unwrap();

        assert_eq!(result.deleted_count, 1);
    }

    #[test]
    fn test_get_all_users() {
        let mock_repo = Arc::new(MockRepo::new());
        let test_user1 = test_user();
        let test_user2 = test_user();

        mock_repo.users.lock().unwrap().push(test_user1);
        mock_repo.users.lock().unwrap().push(test_user2);

        let result = mock_repo.get_all_users().unwrap();

        assert_eq!(result.len(), 2);
    }
}
