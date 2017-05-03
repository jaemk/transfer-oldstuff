/*!
Service
 - initialize external/persistent services/structs

*/
use std::env;
use std::path::Path;
use dotenv::dotenv;
use env_logger;

use diesel::pg::PgConnection;
use r2d2::{Config, Pool};
use r2d2_diesel::ConnectionManager;

use tera::Tera;

use iron::prelude::*;
use iron::typemap::Key;
use iron::middleware::{BeforeMiddleware};
use router::Router;
use logger::Logger;
use persistent::{Write, Read};
use mount::Mount;
use staticfile::Static;

use routes;


type PgPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Copy, Clone)]
pub struct DB;
impl Key for DB { type Value = PgPool; }

#[derive(Copy, Clone)]
pub struct TERA;
impl Key for TERA { type Value = Tera; }

pub struct InfoLog;
impl BeforeMiddleware for InfoLog {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        println!("[{:?}]: {}", req.method, req.url);
        Ok(())
    }
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        Err(err)
    }
}


pub fn establish_connection_pool(database_url: Option<&str>) -> PgPool {
    let db_url = match database_url {
        Some(url) => url.into(), None => {
            dotenv().ok();
            env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
        },
    };
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(config, manager).expect("Failed to create db pool")
}


pub fn start(host: &str, db_url: Option<&str>, log: bool) {
    let host = if host.is_empty() { "localhost:3000" } else { host };

    let db_pool = establish_connection_pool(db_url);
    println!(" ** Established database connection pool **");

    let mut tera = compile_templates!("templates/**/*");
    tera.autoescape_on(vec!["html"]);

    let mut router = Router::new();
    routes::mount(&mut router);

    let mut chain = Chain::new(router);
    chain.link(Write::<DB>::both(db_pool));
    chain.link(Read::<TERA>::both(tera));
    if log {
        env_logger::init().unwrap();
        let (log_before, log_after) = Logger::new(None);
        chain.link_before(log_before);
        chain.link_before(InfoLog);
        chain.link_after(log_after);
    }

    let mut mount = Mount::new();
    mount.mount("/", chain)
         .mount("/static/", Static::new(Path::new("static")));

    println!(" ** Serving at {} **", host);
    Iron::new(mount).http(host).unwrap();
}
