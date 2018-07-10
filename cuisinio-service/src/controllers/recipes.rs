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

    let mongo_client = MongoClient::default();

    let result = mongo_client
        .collection(RECIPE_COLLECTION)
        .insert_one(recipe, None)
        .ok()
        .unwrap();

    let id = match result.inserted_id.unwrap() {
        Bson::ObjectId(id) => id.to_hex(),
        _ => "".to_string(),
    };

    let recipe = read_recipe_from_client(&id, &mongo_client);
    let result = RecipeDto {
        _id: id,
        recipe: recipe,
    };
    Json(result)
}

#[get("/recipe/<id>")]
pub fn get_recipe(id: String) -> Json<RecipeDto> {
    let recipe = read_recipe_from_client(&id, &MongoClient::default());
    let result = RecipeDto {
        _id: id,
        recipe: recipe,
    };
    Json(result)
}

fn read_recipe_from_client(id: &String, client: &MongoClient) -> Recipe {
    let oid = ObjectId::with_string(id).unwrap();
    let filter = doc! {
        "_id" => oid
    };
    let result = client
        .collection(RECIPE_COLLECTION)
        .find_one(Some(filter), None)
        .unwrap()
        .expect(&format!(
            "Recipe with id [{}] was not found in the database!",
            id
        ));
    Recipe::from(result)
}

#[derive(Serialize, Deserialize)]
pub struct BriefRecipeDto {
    pub _id: String,
    pub name: String,
    pub description: Option<String>,
}

#[get("/recipes")]
pub fn list_recipes() -> Json<Vec<BriefRecipeDto>> {
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
            let recipe = Recipe::from(doc);
            BriefRecipeDto {
                _id: id,
                name: recipe.name,
                description: recipe.description,
            }
        })
        .collect();
    Json(result)
}
