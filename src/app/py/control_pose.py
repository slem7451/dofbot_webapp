from Arm_Lib import Arm_Device

Arm = Arm_Device()

#Значения servo 1-5 в градусах
poses = {
    0: [90, 90, 90, 90, 90],
    1: [90, 90, 120, -30, 90],
    2: [90, 0, 180, 0, 90],
    3: [90, 110, -30, 30, 90],
}

def control_pose(*args):
    pose = poses[args[0]]
    pose.append(args[1]) #Добавляем в массив значение servo 6, которое пришло из запроса (чтобы не менять его значение на роботе)
    Arm.Arm_serial_servo_write6_array(pose, 1000)
    return pose