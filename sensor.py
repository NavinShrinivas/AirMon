import serial
import time
import requests
url = "http://localhost:3000/upload"

arduino = serial.Serial(port="/dev/ttyACM0", baudrate=115200, timeout=5)
time.sleep(1.5)
while True:
    if arduino.cts: 
        data =arduino.readline().strip()
    if data:
        str_dat = data.decode()
        print(str_dat)

        # localhost:3000/upload
        # after printing the data,you'll  need to push it to the database 
        if data.decode()[0] == 'C': #meaning data from CO2 sensor
            data_arr = data.decode().split(":")
            print("uploading... ",data_arr[1]);
            upload_dat = "{\"sensor_name\" : \"sensor1\",\"new_data\" : \""+data_arr[1]+"\"}"
            print(upload_dat)
            t = requests.post(url, upload_dat)


    
