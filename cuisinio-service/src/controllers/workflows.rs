use bson::oid::ObjectId;
use bson::Bson;
use rocket_contrib::Json;

use super::RECIPE_COLLECTION;
use model::recipe::Recipe;
use model::workflow::Workflow;
use services::mongo_client::MongoClient;

#[post("/optimise", data = "<recipes>")]
fn optimise_workflow(recipes: Json<Vec<String>>) -> Json<Workflow> {
    let ids = recipes
        .into_inner()
        .iter()
        .map(|id| bson!(ObjectId::with_string(&id).unwrap()))
        .collect();

    let filter = doc! {
        "_id" => doc! {
            "$in" => Bson::Array(ids)
        },
    };

    let affected_recipes: Vec<Recipe> = MongoClient::default()
        .collection(RECIPE_COLLECTION)
        .find(Some(filter), None)
        .unwrap()
        .map(|doc| Recipe::from(doc.unwrap()))
        .collect();

    unimplemented!()
}
