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
extern crate rocket_cors;
extern crate serde;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod controllers;
mod model;
mod services;

use controllers::recipes::*;
use controllers::workflows::*;

fn main() {
    let options = rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount(
            "/",
            routes![get_recipe, new_recipe, list_recipes, optimise_workflow],
        )
        .attach(options)
        .launch();
}
