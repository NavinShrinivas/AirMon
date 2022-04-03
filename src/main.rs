mod routes;
use routes::handler;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};



#[tokio::main]
async fn main()->Result<(),hyper::Error> {
    /*
     *handler::handler_world();
     *println!("Hello, world!");
     */
    let addr = ([127,0,0,1],3000).into();
    //Will convert itself into the next needed type if the needed type knows 
    //how to
    let handler_svc =  make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(handler::service_handler)) });
    //Ahh more complez rust, why canI still not understand this :/

    let server = Server::bind(&addr).serve(handler_svc);
    println!("Listening on port 3000!");
    server.await?;
    Ok(())

}
