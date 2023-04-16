use std::env;
extern crate dotenv;
use crate::models::recipe_model::Recipies;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

// pub trait recipeRepository {
//     fn create_recipe(&self, new_recipe: recipe) -> Result<CustomInsertOneResult, Error>;
//     fn get_recipe(&self, id: &String) -> Result<recipe, Error>;
//     fn update_recipe(&self, id: &String, new_recipe: recipe) -> Result<Box<dyn Any>, Error>;
//     fn delete_recipe(&self, id: &String) -> Result<Box<dyn Any>, Error>;
//     fn get_all_recipes(&self) -> Result<Vec<recipe>, Error>;
// }

// pub enum CustomInsertOneResult {
//     Real(InsertOneResult),
//     Mock(ObjectId),
// }
// #[derive(Debug)]
// pub enum Error {
//     Mongo(mongodb::error::Error),
//     Custom(String),
// }

pub struct MongoRepo {
    col: Collection<Recipies>,
}

// pub struct MockRepo {
//     recipes: Mutex<Vec<recipe>>,
// }

// pub struct MockUpdateResult {
//     pub modified_count: i64,
//     pub matched_count: i64,
//     pub upserted_id: Option<ObjectId>,
// }

// pub struct MockDeleteResult {
//     pub deleted_count: u64,
// }

// impl From<mongodb::error::Error> for Error {
//     fn from(err: mongodb::error::Error) -> Self {
//         Error::Mongo(err)
//     }
// }

// impl From<InsertOneResult> for CustomInsertOneResult {
//     fn from(result: InsertOneResult) -> Self {
//         CustomInsertOneResult::Real(result)
//     }
// }

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("recipes");
        let col: Collection<Recipies> = db.collection("Recipes");
        MongoRepo { col }
    }

    pub fn create_recipe(&self, new_recipe: Recipies) -> Result<InsertOneResult, Error> {
        let new_doc = Recipies {
            id: None,
            name: new_recipe.name,
            ingredients: new_recipe.ingredients,
            instructions: new_recipe.instructions,
        };
        let recipe = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating recipe");
        Ok(recipe)
    }

    pub fn get_recipe(&self, id: &String) -> Result<Recipies, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let recipe_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting recipe's detail");
        Ok(recipe_detail.unwrap())
    }

    fn get_recipe_by_name(&self, name: &String) -> Result<Recipies, Error> {
        let filter = doc! {"name": name};
        let recipe_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting recipe's detail");
        Ok(recipe_detail.unwrap())
        // recipe_detail.ok_or(Error("recipe not found".to_string()))
    }

    pub fn update_recipe(&self, id: &String, new_recipe: Recipies) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_recipe.id,
                    "name": new_recipe.name,
                    "ingredients": new_recipe.ingredients,
                    "instructions": new_recipe.instructions,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating recipe");
        Ok(updated_doc)
    }

    pub fn delete_recipe(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let recipe_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting recipe");
        Ok(recipe_detail)
    }

    pub fn get_all_recipes(&self) -> Result<Vec<Recipies>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of recipes");
        let recipes = cursors.map(|doc| doc.unwrap()).collect();
        Ok(recipes)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mongodb::bson::oid::ObjectId;
//     use std::sync::Arc;

//     fn test_recipe() -> recipe {
//         recipe {
//             id: Some(ObjectId::new()),
//             name: "John Doe".to_string(),
//             location: "New York".to_string(),
//             title: "Software Developer".to_string(),
//         }
//     }

//     #[test]
//     fn test_create_recipe() {
//         let mock_repo = Arc::new(MockRepo::new());
//         let test_recipe = test_recipe();

//         let result = mock_repo.create_recipe(test_recipe).unwrap();

//         if let CustomInsertOneResult::Mock(_) = result {
//             // Test passed, as the result is the Mock variant
//         } else {
//             panic!("Expected Mock variant");
//         }
//     }

//     #[test]
//     fn test_get_recipe() {
//         let mock_repo = Arc::new(MockRepo::new());
//         let test_recipe = test_recipe();
//         let recipe_id = test_recipe.id.clone().unwrap().to_string();

//         mock_repo.recipes.lock().unwrap().push(test_recipe);

//         let result = mock_repo.get_recipe(&recipe_id).unwrap();

//         assert_eq!(result.id.unwrap().to_string(), recipe_id);
//     }

//     #[test]
//     fn test_update_recipe() {
//         let mock_repo = Arc::new(MockRepo::new());
//         let mut test_recipe = test_recipe();
//         let recipe_id = test_recipe.id.clone().unwrap().to_string();

//         mock_repo.recipes.lock().unwrap().push(test_recipe.clone());

//         test_recipe.title = "Senior Software Developer".to_string();
//         let result = mock_repo.update_recipe(&recipe_id, test_recipe).unwrap();
//         let result = result.downcast_ref::<MockUpdateResult>().unwrap();

//         assert_eq!(result.matched_count, 1);
//         assert_eq!(result.modified_count, 1);
//     }

//     #[test]
//     fn test_delete_recipe() {
//         let mock_repo = Arc::new(MockRepo::new());
//         let test_recipe = test_recipe();
//         let recipe_id = test_recipe.id.clone().unwrap().to_string();

//         mock_repo.recipes.lock().unwrap().push(test_recipe);

//         let result = mock_repo.delete_recipe(&recipe_id).unwrap();
//         let result = result.downcast_ref::<MockDeleteResult>().unwrap();

//         assert_eq!(result.deleted_count, 1);
//     }

//     #[test]
//     fn test_get_all_recipes() {
//         let mock_repo = Arc::new(MockRepo::new());
//         let test_recipe1 = test_recipe();
//         let test_recipe2 = test_recipe();

//         mock_repo.recipes.lock().unwrap().push(test_recipe1);
//         mock_repo.recipes.lock().unwrap().push(test_recipe2);

//         let result = mock_repo.get_all_recipes().unwrap();

//         assert_eq!(result.len(), 2);
//     }
// }
