use bson::oid::ObjectId;
use bson::Bson;
use rocket_contrib::Json;

use super::RECIPE_COLLECTION;
use model::recipe::Recipe;
use services::mongo_client::MongoClient;

#[derive(Serialize, Deserialize)]
pub struct RecipeDto {
    pub _id: String,
    pub recipe: Recipe,
}

#[post("/recipe", data = "<recipe>")]
pub fn new_recipe(recipe: Json<Recipe>) -> Json<RecipeDto> {
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

    let id = match result.inserted_id.unwrap() {
        Bson::ObjectId(id) => id.to_hex(),
        _ => "".to_string(),
    };

    get_recipe(id)
}

#[get("/recipe/<id>")]
pub fn get_recipe(id: String) -> Json<RecipeDto> {
    let oid = ObjectId::with_string(&id).unwrap();
    let filter = doc! {
        "_id" => oid 
    };

    let result = MongoClient::default()
        .collection(RECIPE_COLLECTION)
        .find_one(Some(filter), None)
        .unwrap()
        .expect(&format!(
            "Recipe with id [{}] was not found in the database!",
            id
        ));

    let result = Recipe::from(result);
    let result = RecipeDto {
        _id: id,
        recipe: result,
    };

    Json(result)
}

#[get("/recipes")]
pub fn list_recipes() -> Json<Vec<RecipeDto>> {
    let result = MongoClient::default()
        .collection(RECIPE_COLLECTION)
        .find(None, None)
        .unwrap()
        .map(|doc| {
            let doc = doc.unwrap();
            let id = match doc.get("_id").unwrap() {
                Bson::ObjectId(id) => id.to_hex(),
                _ => "".to_string(),
            };
            RecipeDto {
                _id: id,
                recipe: Recipe::from(doc.clone()),
            }
        })
        .collect();
    Json(result)
}
