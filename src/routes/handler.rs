use crate::routes::upload;
use crate::routes::fetch;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode,body };

pub fn handler_world(){
    println!("Hello World, from handler mod!");
    fetch::fetch_world();
    upload::upload_world();
}

pub async fn service_handler(uri : Request<Body>)->Result<Response<Body>, hyper::Error>{
    match(uri.method(),uri.uri().path()){
        (&Method::POST,"/upload")=>{
            let bytes = body::to_bytes(uri.into_body()).await?;
            let new_data = String::from_utf8(bytes.to_vec()).unwrap();
            upload::upload_handler(&new_data);
            Ok(Response::new(Body::from("Data uploaded successfully!")))
        }
        (&Method::GET, "/fetch")=>{
            Ok(Response::new(Body::from("{example data, needs serde JSON evetually}")))
        }
        _ => {
            let mut def_res = Response::default();
            *def_res.status_mut() = StatusCode::NOT_FOUND;//I find this to be a very weird way of modiying the status 
            //of a response, but wat can I do ://
            Ok(def_res)
        }
    }
}
