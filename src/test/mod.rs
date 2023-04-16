#[cfg(test)]
pub mod tests {
    use crate::models::recipe_model::Recipies;

    use super::*;
    use mongodb::bson::oid::ObjectId;
    use rocket::http::uri::Query;
    use std::sync::Arc;

    use mockall::{automock, predicate::*};

    #[cfg_attr(test, automock)]
    pub trait Database {
        fn create_new_recipe(&self, query: String);
    }

    pub fn get_recipe(db: Box<dyn Database>, name: String) {
        
    }

    fn test_recipe() -> Recipies {
        recipe {
            id: Some(ObjectId::new()),
            name: "Toad in the hole".to_string(),
            ingredients: vec!["Flour", "Eggs", "Milk", "Sausages"],
            instructions: vec!["add the flour, eggs, milk into a mixing bowl", "mix by hand or with a stand mixer until it forms a smooth batter", "put the batter in the fridge for 15 mins", "while the batter is cooling fry the sausages in a frying pan until cooked through and browned evenly", "put the sausages into an oven proof dish with sides higher than the sausages", "pour the batter over the sausages until they are just covered", "put in the oven and cook for 35-35 mins"],
        }
    }

    // #[test]
    // fn test_create_recipe() {
    //     let mock_repo = Arc::new(MockRepo::new());
    //     let test_recipe = test_recipe();

    //     let result = mock_repo.create_recipe(test_recipe).unwrap();

    //     if let CustomInsertOneResult::Mock(_) = result {
    //         // Test passed, as the result is the Mock variant
    //     } else {
    //         panic!("Expected Mock variant");
    //     }
    // }

    // #[test]
    // fn test_get_recipe() {
    //     let mock_repo = Arc::new(MockRepo::new());
    //     let test_recipe = test_recipe();
    //     let recipe_id = test_recipe.id.clone().unwrap().to_string();

    //     mock_repo.recipes.lock().unwrap().push(test_recipe);

    //     let result = mock_repo.get_recipe(&recipe_id).unwrap();

    //     assert_eq!(result.id.unwrap().to_string(), recipe_id);
    // }

    // #[test]
    // fn test_update_recipe() {
    //     let mock_repo = Arc::new(MockRepo::new());
    //     let mut test_recipe = test_recipe();
    //     let recipe_id = test_recipe.id.clone().unwrap().to_string();

    //     mock_repo.recipes.lock().unwrap().push(test_recipe.clone());

    //     test_recipe.title = "Senior Software Developer".to_string();
    //     let result = mock_repo.update_recipe(&recipe_id, test_recipe).unwrap();
    //     let result = result.downcast_ref::<MockUpdateResult>().unwrap();

    //     assert_eq!(result.matched_count, 1);
    //     assert_eq!(result.modified_count, 1);
    // }

    // #[test]
    // fn test_delete_recipe() {
    //     let mock_repo = Arc::new(MockRepo::new());
    //     let test_recipe = test_recipe();
    //     let recipe_id = test_recipe.id.clone().unwrap().to_string();

    //     mock_repo.recipes.lock().unwrap().push(test_recipe);

    //     let result = mock_repo.delete_recipe(&recipe_id).unwrap();
    //     let result = result.downcast_ref::<MockDeleteResult>().unwrap();

    //     assert_eq!(result.deleted_count, 1);
    // }

    // #[test]
    // fn test_get_all_recipes() {
    //     let mock_repo = Arc::new(MockRepo::new());
    //     let test_recipe1 = test_recipe();
    //     let test_recipe2 = test_recipe();

    //     mock_repo.recipes.lock().unwrap().push(test_recipe1);
    //     mock_repo.recipes.lock().unwrap().push(test_recipe2);

    //     let result = mock_repo.get_all_recipes().unwrap();

    //     assert_eq!(result.len(), 2);
    // }
}
