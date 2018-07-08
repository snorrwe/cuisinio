use rocket_contrib::Json;

use super::RECIPE_COLLECTION;
use model::recipe::Recipe;
use services::mongo_client::MongoClient;

#[post("/recipe", data = "<recipe>")]
pub fn new_recipe(recipe: Json<Recipe>) -> String {
    let recipe = recipe.into_inner();
    if !recipe.is_valid() {
        panic!("Recipe is not valid!")
    }
    let recipe = bson!(recipe);
    let result = MongoClient::default()
        .collection(RECIPE_COLLECTION)
        .insert_one(recipe, None)
        .ok()
        .unwrap();
    format!("{{\"recipe_id\": {}}}", result.inserted_id.unwrap())
}

#[get("/recipes")]
pub fn list_recipes() -> Json<Vec<Recipe>> {
    let result = MongoClient::default()
        .collection(RECIPE_COLLECTION)
        .find(None, None)
        .unwrap()
        .map(|doc| {
            let doc = doc.unwrap();
            Recipe::from(doc)
        })
        .collect();
    Json(result)
}
