# AirMon 

> This Rust piece of code is not at the level I would usually have it be, feel free to refactor and rewrite em if you want to :).

## About AimMon


What is this AirMon, well its a back end ONLY Rust code that has the ability to collect data from multiple sensors simultaneoulsy (Tokio async).

It creates neat API calls for both backed sensors to upload and frontend to fetch. So what's the structure for these API's?
A few CURL API calls should clear it up.
Uploading data : 
```
Query example : curl -X POST localhost:8080/upload -d '{"sensor_name":"sensor1" , "new_data":"688"}'
Reply example : {'status' : 'true'}
```
Fetching data:
```
Query example : curl -X GET localhost:8080/fetch -d '{"sensor_name":"sensor1"}'
Reply example : {'status' : 'true' , 'content' : '685'}

Reply example2 : {'status' : 'false' }
```

wanna try out the API and see how it works? Follow this after having rust toolchain installed :
```
$ git clone https://github.com/NavinShrinivas/AirMon.git 
$ cd AirMon
$ cargo run
```
