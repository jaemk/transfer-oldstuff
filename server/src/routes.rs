
use router::Router;
use handlers;


pub fn mount(router: &mut Router) {
    router.get("/", handlers::home, "home");
}