use rocket_contrib::Json;

#[post("/optimise", data = "<recipes>")]
fn optimise_workflow(recipes: Json<Vec<String>>) -> String {
    unimplemented!()
}
