use crate::{models::recipe_model::Recipies, repository::mongodb_recipe::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/recipe", data = "<new_recipe>")]
pub fn create_recipe(
    db: &State<MongoRepo>,
    new_recipe: Json<Recipies>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Recipies {
        id: None,
        name: new_recipe.name.clone(),
        ingredients: new_recipe.ingredients.clone(),
        instructions: new_recipe.instructions.clone(),
    };
    let recipe_detail = db.create_recipe(data);
    match recipe_detail {
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/recipe/<path>")]
pub fn get_recipe(db: &State<MongoRepo>, path: String) -> Result<Json<Recipies>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let recipe_detail = db.get_recipe(&id);
    match recipe_detail {
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/recipe/<path>", data = "<new_recipe>")]
pub fn update_recipe(
    db: &State<MongoRepo>,
    path: String,
    new_recipe: Json<Recipies>,
) -> Result<Json<Recipies>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Recipies {
        id: None,
        name: new_recipe.name.clone(),
        ingredients: new_recipe.ingredients.clone(),
        instructions: new_recipe.instructions.clone(),
    };
    let update_result = db.update_recipe(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_recipe_info = db.get_recipe(&id);
                return match updated_recipe_info {
                    Ok(recipe) => Ok(Json(recipe)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/recipe/<path>")]
pub fn delete_recipe(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_recipe(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("recipe successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/recipes")]
pub fn get_all_recipes(db: &State<MongoRepo>) -> Result<Json<Vec<Recipies>>, Status> {
    let recipes = db.get_all_recipes();
    match recipes {
        Ok(recipes) => Ok(Json(recipes)),
        Err(_) => Err(Status::InternalServerError),
    }
}
