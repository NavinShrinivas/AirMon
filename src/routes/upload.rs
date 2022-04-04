//The sensors want to upload data
use serde_json::Value;
use std::fs;
use std::io::Write;
use hyper::Body;

pub fn upload_handler(new_data: &str)->Body {
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
    Body::from("{'status' : 'true'}")
}
