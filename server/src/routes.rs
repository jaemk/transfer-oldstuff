
use router::Router;
use handlers;


pub fn mount(router: &mut Router) {
    router.get("/", handlers::home, "home");
    router.post("/upload", handlers::upload, "upload");
    router.get("/access", handlers::access, "access");
    router.get("/download/:key", handlers::download, "download");
}
