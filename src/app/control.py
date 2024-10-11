from Arm_Lib import Arm_Device

Arm = Arm_Device()

def control(*args):
    servo = args[0]
    angle = args[1]
    Arm.Arm_serial_servo_write(servo, angle, 1000)