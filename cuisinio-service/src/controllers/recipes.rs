use bson::ordered::OrderedDocument;
use bson::Bson;
use rocket_contrib::Json;

use model::recipe::Recipe;
use services::mongo_client::MongoClient;

#[post("/recipe", data = "<recipe>")]
pub fn new_recipe(recipe: Json<Recipe>) -> String {
    let recipe = recipe.into_inner();
    let recipe = document_from_recipe(recipe);
    let result = MongoClient::default()
        .collection("recipes")
        .insert_one(recipe, None)
        .ok()
        .unwrap();

    format!("{{\"recipe_id\": {}}}", result.inserted_id.unwrap())
}

fn document_from_recipe(recipe: Recipe) -> OrderedDocument {
    let steps = steps_from_recipe(&recipe);
    doc! {
        "name" => recipe.name,
        "description" => match recipe.description {
            Some(d) => d,
            None => "".to_string(),
        },
        "steps" => steps,
    }
}

fn steps_from_recipe(recipe: &Recipe) -> Bson {
    let steps = recipe
        .steps
        .iter()
        .map(|s| {
            let doc = doc! {
                "name" => s.name.clone(),
                "description" => match s.description {
                    Some(ref d) => d.clone(),
                    None => "".to_string(),
                },
                "duration_ms" => s.duration_ms,
            };
            bson!(doc)
        })
        .collect();
    Bson::Array(steps)
}

#[get("/recipes")]
pub fn list_recipes() -> Json<Vec<Recipe>> {
    let client = MongoClient::default();
    let collection = client.collection("recipes");

    let result = collection.find(None, None).unwrap();

    unimplemented!()
}
