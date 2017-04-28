
use iron::prelude::*;
use iron::status;


pub fn home(req: &mut Request) -> IronResult<Response> {
    let content_type = mime!(Text/Html);
    Ok(Response::with((content_type, status::Ok, "hey")))
}
