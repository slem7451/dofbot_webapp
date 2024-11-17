from Arm_Lib import Arm_Device
import json

Arm = Arm_Device()

def trajectory(*args):
    data = args[0]
    servo6 = args[1]

    data = json.loads(data[2:-1].encode().decode('unicode_escape'))['arrays']

    for item in data:
        pose = [None] * 6
        pose[0] = item[0]
        pose[1] = item[1]
        pose[2] = item[2]
        pose[3] = item[3]
        pose[4] = item[4]
        if item[5] <= 180 and item[5] >= -180:
            servo6 = item[5]
        pose[5] = int(servo6)
        time = item[6]
        Arm.Arm_serial_servo_write6_array(pose, time)

    return pose