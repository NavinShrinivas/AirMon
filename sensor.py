import serial
import time
arduino = serial.Serial(port="/dev/ttyACM0", baudrate=115200, timeout=.1)
while True:
    time.sleep(0.5)
    data = arduino.readline()
    print(data)
