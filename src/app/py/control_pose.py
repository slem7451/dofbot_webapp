from Arm_Lib import Arm_Device

Arm = Arm_Device()
poses = {
    0: [90, 90, 90, 90, 90, 90],
    1: [90, 90, 120, -30, 90, 90],
    2: [90, 0, 180, 0, 90, 90],
    3: [90, 110, -30, 30, 90, 90],
}

def control_pose(*args):
    pose = poses[args[0]]
    Arm.Arm_serial_servo_write6_array(pose, 1000)