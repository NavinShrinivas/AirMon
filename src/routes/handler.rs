use hyper;
use hyper::{body, Body,Request, Response};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub async fn home_handler(uri: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let res_body = Body::from("You have reached AirMon's home endpoint!");
    let res_builder = Response::builder().body(res_body);
    return Ok(res_builder.unwrap());
}


pub async fn fetch_handler(uri: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let params: HashMap<String, String> = uri
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
    .unwrap_or_else(HashMap::new);
    println!("{:?}",params);
    let res_body = Body::from(fetch_handler_fn(params));
    let res_builder = Response::builder().body(res_body);
    return Ok(res_builder.unwrap());
}
pub async fn fetch_temp_handler(uri: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let params: HashMap<String, String> = uri
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
    .unwrap_or_else(HashMap::new);
    println!("{:?}",params);
    let res_body = Body::from(fetch_temp_handler_fn(params));
    let res_builder = Response::builder().body(res_body);
    return Ok(res_builder.unwrap());
}



pub async fn upload_handler(uri: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let bytes = body::to_bytes(uri.into_body()).await?;
    let new_data = String::from_utf8(bytes.to_vec()).unwrap();
    println!("{:?}",new_data);
    let res_body = Body::from(upload_handler_fn(&new_data));
    let res_builder = Response::builder().body(res_body);
    return Ok(res_builder.unwrap());
}


pub async fn upload_temp_handler(uri: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let bytes = body::to_bytes(uri.into_body()).await?;
    let new_data = String::from_utf8(bytes.to_vec()).unwrap();
    println!("{:?}",new_data);
    let res_body = Body::from(upload_temp_handler_fn(&new_data));
    let res_builder = Response::builder().body(res_body);
    return Ok(res_builder.unwrap());
}




pub fn fetch_handler_fn(req_body: HashMap<String,String>) -> Body {
    let mut file_fd: File = match File::open(format!("{}{}", req_body["sensor_number"], ".txt"))
    {
        Ok(fd) => fd,
        Err(_) => {
            let json_as_str = format!("{}", "{\"status\" : false }");
            return Body::from(json_as_str);
        }
    };
    let mut data_raw: String = String::new();
    file_fd.read_to_string(&mut data_raw).expect("read error!");
    let data_vec: Vec<&str> = data_raw.trim().split("\n").collect();
    let mut present_values: i32 = 0;
    let mut sum: i32 = 0;
    for (i, value) in data_vec.iter().enumerate() {
        present_values = i as i32;
        let parsed_values: i32 = value.parse().unwrap();
        sum += parsed_values;
    }
    let avg = sum / (present_values + 1);

    let json_as_str = format!(
        "{}{}{}", //this ' is not misplaced
        "{\"status\" : true , \"content\" : ",
        avg.to_string(),
        "}"
    );
    Body::from(json_as_str)
}
pub fn fetch_temp_handler_fn(req_body: HashMap<String,String>) -> Body {
    let mut file_fd: File = match File::open(format!("{}{}{}", req_body["sensor_number"], "_temp",".txt"))
    {
        Ok(fd) => fd,
        Err(_) => {
            let json_as_str = format!("{}", "{\"status\" : false }");
            return Body::from(json_as_str);
        }
    };
    let mut data_raw: String = String::new();
    file_fd.read_to_string(&mut data_raw).expect("read error!");
    let data_vec: Vec<&str> = data_raw.trim().split("\n").collect();
    let mut present_values: f32 = 0.0;
    let mut sum: f32 = 0.0;
    for (i, value) in data_vec.iter().enumerate() {
        present_values = i as f32;
        let parsed_values: f32 = value.parse().unwrap();
        sum += parsed_values;
    }
    let avg = sum / (present_values + 1.0);

    let json_as_str = format!(
        "{}{}{}", //this ' is not misplaced
        "{\"status\" : true , \"content\" : ",
        avg.to_string(),
        "}"
    );
    Body::from(json_as_str)
}




pub fn upload_handler_fn(new_data: &str) -> Body {
    let json_object: Value = serde_json::from_str(&new_data).unwrap();
    println!(
        "upload log : from sensor : {}, new data : {}",
        json_object["sensor_name"], json_object["new_data"]
    );
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(format!(
                "{}{}",
                json_object["sensor_name"].as_str().unwrap(),
                ".txt"
        ))
        .expect("cannot open file");
    file.write_all(json_object["new_data"].as_str().unwrap().as_bytes())
        .expect("write failed");
    file.write_all("\n".as_bytes()).expect("Write error");
    Body::from("{\"status\" : true}")
}

pub fn upload_temp_handler_fn(new_data: &str) -> Body {
    let json_object: Value = serde_json::from_str(&new_data).unwrap();
    println!(
        "upload log : from sensor : {}, new data : {}",
        json_object["sensor_name"], json_object["new_data"]
    );
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(format!(
                "{}{}{}",
                json_object["sensor_name"].as_str().unwrap(),
                "_temp",
                ".txt"
        ))
        .expect("cannot open file");
    file.write_all(json_object["new_data"].as_str().unwrap().as_bytes())
        .expect("write failed");
    file.write_all("\n".as_bytes()).expect("Write error");
    Body::from("{\"status\" : true}")
}
