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

use rocket_contrib::Json;

mod controllers;
mod model;
mod services;
use controllers::recipes::*;

#[post("/optimise", data = "<recipes>")]
fn optimise_workflow(recipes: Json<Vec<String>>) -> String {
    unimplemented!()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![new_recipe, list_recipes, optimise_workflow])
        .launch();
}
