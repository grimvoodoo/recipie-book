use crate::{models::recipe_model::Recipes, repository::mongodb_recipe::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/recipe/<test>", data = "<new_recipe>")]
pub fn create_recipe(
    db: &State<MongoRepo>,
    new_recipe: Json<Recipes>,
    test: bool,
) -> Result<Json<InsertOneResult>, Status> {
    let recipe_detail = db.create_recipe(new_recipe.into_inner(), test);
    match recipe_detail {
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/recipe/<test>/<path>")]
pub fn get_recipe(
    db: &State<MongoRepo>,
    path: String,
    test: bool,
) -> Result<Json<Recipes>, Status> {
    let name = path;
    if name.is_empty() {
        return Err(Status::BadRequest);
    };
    let recipe_detail = db.get_recipe(&name, test);
    match recipe_detail {
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/recipe/<test>/<path>", data = "<new_recipe>")]
pub fn update_recipe(
    db: &State<MongoRepo>,
    path: String,
    new_recipe: Json<Recipes>,
    test: bool,
) -> Result<Json<Recipes>, Status> {
    let name = path;
    if name.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Recipes {
        id: None,
        name: new_recipe.name.clone(),
        ingredients: new_recipe.ingredients.clone(),
        instructions: new_recipe.instructions.clone(),
    };
    let update_result = db.update_recipe(&name, data, test);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_recipe_info = db.get_recipe(&name, test);
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

#[delete("/recipe/<test>/<path>")]
pub fn delete_recipe(
    db: &State<MongoRepo>,
    path: String,
    test: bool,
) -> Result<Json<&str>, Status> {
    let name = path;
    if name.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_recipe(&name, test);
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

#[get("/recipes/<test>")]
pub fn get_all_recipes(db: &State<MongoRepo>, test: bool) -> Result<Json<Vec<Recipes>>, Status> {
    let recipes = db.get_all_recipes(test);
    match recipes {
        Ok(recipes) => Ok(Json(recipes)),
        Err(_) => Err(Status::InternalServerError),
    }
}
