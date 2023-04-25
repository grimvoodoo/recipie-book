use crate::{models::recipe_model::Recipes, repository::mongodb_recipe::MongoRepo};

fn test_create_recipe() {
    let mongo_repo = MongoRepo::init();

    let new_recipe = Recipes::default();
    let result = mongo_repo.create_recipe(new_recipe, true);

    match result {
        Ok(insert_result) => {
            println!("Inserted recipe with ID: {:?}", insert_result.inserted_id);
        }
        Err(err) => {
            panic!("Error creating recipe: {}", err);
        }
    }
}

pub fn test_get_recipe() {
    let mongo_repo = MongoRepo::init();

    let result = mongo_repo.get_recipe(&String::from("Pancakes"), true);

    match result {
        Ok(get_result) => {
            println!("Get Recipe with name: {:?}", get_result);
        }
        Err(err) => {
            panic!("Error creating recipe: {}", err);
        }
    }
}

pub fn test_get_all_recipes() {
    let mongo_repo = MongoRepo::init();

    let result = mongo_repo.get_all_recipes(true);

    match result {
        Ok(get_result) => {
            println!("Get all recipes: {:?}", get_result[0]);
        }
        Err(err) => {
            panic!("Error creating recipe: {}", err);
        }
    }
}

pub fn test_update_recipe() {
    let mongo_repo = MongoRepo::init();

    let updated_recipe = Recipes::default();
    let updated_recipe = Recipes {
        name: String::from("Toad in the hole"),
        ..updated_recipe
    };
    let result = mongo_repo.update_recipe(&String::from("Pancakes"), updated_recipe, true);

    match result {
        Ok(update_result) => {
            println!("updated name to: {:?}", update_result);
        }
        Err(err) => {
            panic!("Error creating recipe: {}", err);
        }
    }
}

pub fn delete_recipe() {
    let mongo_repo = MongoRepo::init();

    let result = mongo_repo.delete_recipe(&String::from("Toad in the hole"), true);

    match result {
        Ok(delete_result) => {
            println!("deleted recipe: {:?}", delete_result);
        }
        Err(err) => {
            panic!("Error creating recipe: {}", err);
        }
    }
}
#[test]
pub fn crud_test() {
    test_create_recipe();
    test_get_recipe();
    test_get_all_recipes();
    test_update_recipe();
    delete_recipe();
}
