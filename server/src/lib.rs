#![recursion_limit = "1024"]
#[macro_use] extern crate error_chain;

extern crate dotenv;
extern crate chrono;
extern crate rand;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;


#[macro_use] extern crate tera;

#[macro_use] extern crate mime;
extern crate params;
extern crate env_logger;

extern crate iron;
#[macro_use] extern crate router;
extern crate logger;
extern crate persistent;
extern crate mount;
extern crate staticfile;

pub mod errors {
    error_chain! {}
}

pub mod service;
pub mod handlers;
pub mod routes;
pub mod schema;
pub mod models;
