
use crate::routes::fetch;
use crate::routes::upload;
use hyper;
use hyper::{body, Body, Method, Request, Response, StatusCode};

pub async fn service_handler(uri: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (uri.method(), uri.uri().path()) {
        (&Method::POST, "/upload") => {
            let bytes = body::to_bytes(uri.into_body()).await?;
            let new_data = String::from_utf8(bytes.to_vec()).unwrap();
            let res_body : Body = upload::upload_handler(&new_data);
            Ok(Response::new(res_body))
        }
        (&Method::GET, "/fetch") => {
            let bytes = body::to_bytes(uri.into_body()).await?;
            let new_data = String::from_utf8(bytes.to_vec()).unwrap();
            let res_body: Body = fetch::fetch_handler(new_data);
            let res_builder = Response::builder().header("AccessControlAllowOrigin","Any").body(res_body);
            return Ok(res_builder.unwrap());
        }
        _ => {
            let mut def_res = Response::default();
            *def_res.status_mut() = StatusCode::NOT_FOUND; //I find this to be a very weird way of modiying the status
                                                           //of a response, but wat can I do ://
            Ok(def_res)
        }
    }
}
