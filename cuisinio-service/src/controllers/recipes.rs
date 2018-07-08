use bson;
use rocket_contrib::Json;

use model::recipe::Recipe;
use services::mongo_client::MongoClient;

#[post("/recipe", data = "<recipe>")]
pub fn new_recipe(recipe: Json<Recipe>) -> String {
    let recipe = recipe.into_inner();
    let rec = recipe.clone();
    let recipe_doc = doc! {
        "name" => rec.name,
        "description" => match rec.description {
            Some(d) => d,
            None => "".to_string(),
        }
    };
    let client = MongoClient::default();
    let collection = client.collection("recipes");
    let recipe_result = collection.insert_one(recipe_doc, None).ok().unwrap();
    insert_steps(
        &recipe,
        &recipe_result
            .clone()
            .inserted_id
            .expect("Inserted Id should not be None!"),
    );

    format!("{{\"recipe_id\": {}}}", recipe_result.inserted_id.unwrap())
}

fn insert_steps(recipe: &Recipe, recipe_id: &bson::Bson) {
    let steps: Vec<bson::ordered::OrderedDocument> = recipe
        .steps
        .iter()
        .map(|ref s| {
            doc! {
                "name" => s.name.clone(),
                "description" => match s.description {
                    Some(ref d) => d.clone(),
                    None => "".to_string(),
                },
                "duration_ms" => s.duration_ms,
                "recipe_id" => recipe_id.clone(),
            }
        })
        .collect();
    let client = MongoClient::default();
    let collection = client.collection("steps");
    collection.insert_many(steps, None).ok().unwrap();
}

#[get("/recipes")]
pub fn list_recipes() -> Json<Vec<Recipe>> {
    let client = MongoClient::default();
    let collection = client.collection("recipes");

    let result = collection.find(None, None).unwrap();

    unimplemented!()
}
