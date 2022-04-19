extern crate hyper;

mod routes;
use hyper::Body;
use hyper::Server;
use routerify::{Router, RouterService};
use routerify_cors::enable_cors_all;
use routes::handler;
// Create a router.
fn router() -> Router<Body, hyper::Error> {
    Router::builder()
        // Attach the CORS middleware.
        .middleware(enable_cors_all())
        .get("/", handler::home_handler)
        .get("/fetch",handler::fetch_handler)
        .post("/upload",handler::upload_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    /*
     *handler::handler_world();
     *println!("Hello, world!");
     */
    let addr = ([127, 0, 0, 1], 3000).into();
    //Will convert itself into the next needed type if the needed type knows, ie IP addr
    let router = router();
    let service = RouterService::new(router).unwrap();

    let server = Server::bind(&addr).serve(service);
    println!("Listening on port 8080!");
    server.await?;
    Ok(())
}
