import serial
import time
import requests
arduino = serial.Serial(port="/dev/ttyACM0", baudrate=115200, timeout=.1)
url = "localhost:3000/upload"
while True:
    time.sleep(0.5)
    data = arduino.readline()
    print(data)

    # localhost:3000/upload
    # after printing the data,you'll  need to push it to the database 
    t = requests.post(url, data)
    
