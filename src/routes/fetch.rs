//The frontend wants to fetch data

use hyper::Body;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

pub fn fetch_handler(req_body: String) -> Body {
    let json_object: Value = serde_json::from_str(&req_body).unwrap();
    let mut file_fd: File = match File::open(format!(
        "{}{}",
        json_object["sensor_name"].as_str().unwrap(),
        ".txt"
    )) {
        Ok(fd) => fd,
        Err(_) => {
            let json_as_str = format!("{}", "{'status' : 'false' }");
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
        "{}{}'{}",
        "{'status' : 'true' , 'content' : '",
        avg.to_string(),
        "}"
    );

    Body::from(json_as_str)
}
