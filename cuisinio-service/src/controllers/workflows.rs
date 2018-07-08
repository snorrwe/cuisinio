use bson::oid::ObjectId;
use bson::Bson;
use rocket_contrib::Json;

use model::recipe::Recipe;
use services::mongo_client::MongoClient;

const RECIPE_COLLECTION: &str = "recipes"; // TODO: abstract this

#[post("/optimise", data = "<recipes>")]
fn optimise_workflow(recipes: Json<Vec<String>>) -> String {
    let ids: Vec<ObjectId> = recipes
        .into_inner()
        .iter()
        .map(|id| ObjectId::with_string(&id).unwrap())
        .collect();

    let filter = doc!{
        "_id" => doc! {
            "$in" => bson!(ids)
        },
    };

    let result: Vec<Recipe> = MongoClient::default()
        .collection(RECIPE_COLLECTION)
        .find(None, None)
        .unwrap()
        .map(|doc| {
            let doc = doc.unwrap();
            Recipe::from(doc)
        })
        .collect();
    unimplemented!()
}
