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
extern crate serde;

mod controllers;
mod model;
mod services;

use controllers::recipes::*;
use controllers::workflows::*;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![get_recipe, new_recipe, list_recipes, optimise_workflow],
        )
        .launch();
}
