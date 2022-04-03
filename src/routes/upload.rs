//The sensors want to upload data

use serde_json::{Result,Value};

pub fn upload_world(){
    println!("Hello world, From upload mod!");
}


pub fn upload_handler(new_data : &str){
   let json_object : Value = serde_json::from_str(&new_data).unwrap();
   println!("upload log : from sensor : {}, new data : {}",json_object["sensor_name"],json_object["new_data"]);
}
