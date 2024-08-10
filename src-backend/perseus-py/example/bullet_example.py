import pybullet as bullet
import time
import pybullet_data
physicsClient = bullet.connect(bullet.GUI)#or p.DIRECT for non-graphical version
bullet.setAdditionalSearchPath(pybullet_data.getDataPath()) #optionally
bullet.setGravity(0,0,-10)
planeId = bullet.loadMJCF("/home/huangsu/work/github/robot/mujoco_menagerie/bitcraze_crazyflie_2/scene.xml")

startPos = [0,0,2]
startOrientation = bullet.getQuaternionFromEuler([0,0,0])
boxId = bullet.loadURDF("r2d2.urdf",startPos, startOrientation)
#set the center of mass frame (loadURDF sets base link frame) startPos/Ornp.resetBasePositionAndOrientation(boxId, startPos, startOrientation)
for i in range (10000):
    bullet.stepSimulation()
    time.sleep(1./240.)
cubePos, cubeOrn = bullet.getBasePositionAndOrientation(boxId)
# print(cubePos,cubeOrn)
bullet.disconnect()

