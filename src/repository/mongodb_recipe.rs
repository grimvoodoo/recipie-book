use std::env;
extern crate dotenv;
use crate::models::recipe_model::Recipes;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Recipes>,
    test_col: Collection<Recipes>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).expect("Could not connect with client uri");
        let db = client.database("recipes");
        let col: Collection<Recipes> = db.collection("Recipes");

        let test_uri = match env::var("MONGOURI_TEST") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let test_client = Client::with_uri_str(test_uri).unwrap();
        let test_db = test_client.database("recipes");
        let test_col: Collection<Recipes> = test_db.collection("Recipes");
        MongoRepo { col, test_col }
    }

    pub fn create_recipe(&self, new_recipe: Recipes, test: bool) -> Result<InsertOneResult, Error> {
        let new_doc = Recipes {
            id: None,
            name: new_recipe.name,
            ingredients: new_recipe.ingredients,
            instructions: new_recipe.instructions,
        };
        if test == true {
            let recipe = self
                .test_col
                .insert_one(new_doc, None)
                .ok()
                .expect("Error creating recipe");
            Ok(recipe)
        } else {
            let recipe = self
                .col
                .insert_one(new_doc, None)
                .ok()
                .expect("Error creating recipe");
            Ok(recipe)
        }
    }

    pub fn get_recipe(&self, name: &String, test: bool) -> Result<Recipes, Error> {
        // let obj_name = ObjectId::parse_str(name).unwrap();
        let filter = doc! {"name": name};
        if test == true {
            let recipe_detail = self
                .test_col
                .find_one(filter, None)
                .ok()
                .expect("Error getting recipe's detail");
            Ok(recipe_detail.unwrap())
        } else {
            let recipe_detail = self
                .col
                .find_one(filter, None)
                .ok()
                .expect("Error getting recipe's detail");
            Ok(recipe_detail.unwrap())
        }
    }

    fn get_recipe_by_name(&self, name: &String, test: bool) -> Result<Recipes, Error> {
        let filter = doc! {"name": name};
        if test == true {
            let recipe_detail = self
                .test_col
                .find_one(filter, None)
                .ok()
                .expect("Error getting recipe's detail");
            Ok(recipe_detail.unwrap())
        } else {
            let recipe_detail = self
                .col
                .find_one(filter, None)
                .ok()
                .expect("Error getting recipe's detail");
            Ok(recipe_detail.unwrap())
        }
    }

    pub fn update_recipe(
        &self,
        name: &String,
        new_recipe: Recipes,
        test: bool,
    ) -> Result<UpdateResult, Error> {
        // let obj_name = ObjectId::parse_str(name).unwrap();
        let filter = doc! {"name": name};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_recipe.id,
                    "name": new_recipe.name,
                    "ingredients": new_recipe.ingredients,
                    "instructions": new_recipe.instructions,
                },
        };
        if test == true {
            let updated_doc = self
                .test_col
                .update_one(filter, new_doc, None)
                .ok()
                .expect("Error updating recipe");
            Ok(updated_doc)
        } else {
            let updated_doc = self
                .col
                .update_one(filter, new_doc, None)
                .ok()
                .expect("Error updating recipe");
            Ok(updated_doc)
        }
    }

    pub fn delete_recipe(&self, name: &String, test: bool) -> Result<DeleteResult, Error> {
        // let obj_name = ObjectId::parse_str(name).unwrap();
        let filter = doc! {"name": name};
        if test == true {
            let recipe_detail = self
                .test_col
                .delete_one(filter, None)
                .ok()
                .expect("Error deleting recipe");
            Ok(recipe_detail)
        } else {
            let recipe_detail = self
                .col
                .delete_one(filter, None)
                .ok()
                .expect("Error deleting recipe");
            Ok(recipe_detail)
        }
    }

    pub fn get_all_recipes(&self, test: bool) -> Result<Vec<Recipes>, Error> {
        if test == true {
            let cursors = self
                .test_col
                .find(None, None)
                .ok()
                .expect("Error getting list of recipes");
            let recipes = cursors.map(|doc| doc.unwrap()).collect();
            Ok(recipes)
        } else {
            let cursors = self
                .col
                .find(None, None)
                .ok()
                .expect("Error getting list of recipes");
            let recipes = cursors.map(|doc| doc.unwrap()).collect();
            Ok(recipes)
        }
    }
}
