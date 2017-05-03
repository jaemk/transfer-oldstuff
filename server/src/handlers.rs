
use std::fs;
use std::io::Read;

use diesel;
use diesel::prelude::*;

use iron::prelude::*;
use iron::status;
use persistent::{Read as PerRead, Write};
use rand::{self, Rng};
use serde_json;

use tera::Context;
use service::TERA;
use service::DB;
use models;


/// Macro to pull a pooled db connection out a request typemap
macro_rules! get_dbconn {
    ($request:expr) => {
        {
            let mutex = $request.get::<Write<DB>>().unwrap();
            let pool = mutex.lock().unwrap().clone();
            pool.get().unwrap()
        }
    }
}


/// Macro to pull our template renderer out of a request typemap
macro_rules! get_templates {
    ($request:expr) => {
        {
            let arc = $request.get::<PerRead<TERA>>().unwrap();
            arc.clone()
        }
    }
}


/// Generate a new random key
fn gen_key(n_chars: usize) -> String {
    rand::thread_rng()
        .gen_ascii_chars()
        .take(n_chars)
        .collect::<String>()
}


#[derive(Deserialize)]
struct Input {
    bytes: Vec<u8>,
    iv: Vec<u8>,
}

pub fn upload(req: &mut Request) -> IronResult<Response> {
    use schema::items;
    use schema::items::dsl::*;
    let conn = get_dbconn!(req);

    let mut content = String::new();
    req.body.read_to_string(&mut content).expect("failed reading req.body");
    let content: Input = serde_json::from_str(&content).expect("failed parsing");

    let new_key = gen_key(5);
    let new_item = models::NewItem {
        unique_key: &new_key,
        iv: &content.iv,
        bytes: &content.bytes,
        lifespan: 100000,
        dl_limit: 5,
    };

    let new_item = diesel::insert(&new_item).into(items::table).get_result::<models::Item>(&*conn)
        .expect("Failed inserting item");

    let resp = json!({ "ok": "ok" });
    let content_type = mime!(Application/Json);
    Ok(Response::with((content_type, status::Ok, resp.to_string())))
}

pub fn home(req: &mut Request) -> IronResult<Response> {
    let templates = get_templates!(req);
    let mut context = Context::new();
    let name = "James".to_string();
    context.add("name", &name);
    let content = templates.render("core/home.html", &context).unwrap();
    let content_type = mime!(Text/Html);
    Ok(Response::with((content_type, status::Ok, content)))
}
