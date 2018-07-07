#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};
use rocket_contrib::Json;

mod recipe;
use recipe::Recipe;

#[post("/recipe", data = "<recipe>")]
fn new_recipe(recipe: Json<Recipe>) -> String {
    let client = Client::connect("localhost", 27017)
        .ok()
        .expect("Failed to initialize client.");

    let db = client.db("cuisinio");
    // match db.create_collection("recipes", None) {
    //     Ok(result) => result,
    //     Err(error) => panic!("Failed to create recipes collection {:?}", error),
    // };

    let recipe = recipe.into_inner();
    let recipe_doc = doc! {
        "name" => recipe.name,
        "description" => match recipe.description {
            Some(d) => d,
            None => "".to_string(),
        }
    };
    let collection = db.collection("recipes");
    let result = collection.insert_one(recipe_doc, None).ok().unwrap();
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
                "recipe_id" => result.clone().inserted_id.expect("wtf?"),
            }
        })
        .collect();
    let collection = db.collection("steps");
    let result = collection.insert_many(steps, None).ok().unwrap();

    "ok".to_string() // TODO
}

#[get("/recipes")]
fn list_recipes() -> Json<Vec<Recipe>> {
    let client = Client::connect("localhost", 27017)
        .ok()
        .expect("Failed to initialize client.");

    let db = client.db("cuisinio");
    let collection = db.collection("recipes");

    let result = collection.find(None, None);

    // println!("{:?}", result);

    unimplemented!()
}

#[post("/optimise", data = "<recipes>")]
fn optimise_workflow(recipes: Json<Vec<Recipe>>) -> String {
    unimplemented!()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![new_recipe, optimise_workflow])
        .launch();
}
