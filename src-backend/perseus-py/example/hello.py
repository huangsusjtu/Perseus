import random

import perseus

perseus.init("/home/huangsu/work/Perseus/src-backend/libformat/asset/opendrive",
             "/home/huangsu/work/Perseus/src-backend/libmodel/asset/sanitation")

### 地图相关
map_list = perseus.list_map()
# print(map_list)

map_name = random.choice(map_list)
map_name = "mixed_roads.xodr"
print(map_name)
map_data = perseus.get_map(map_name)

roads = map_data.get_roads()
print(roads, len(roads))
junctions = map_data.get_junctions()
print(junctions, len(junctions))
sites = map_data.get_sites()
print(sites, len(sites))
areas = map_data.get_areas()
print(areas, len(areas))

### 场景相关
scene_list = perseus.list_scenario()
print("scene_list", scene_list)
scene_name = random.choice(scene_list)
scene_data = perseus.get_scenario(scene_name)
print("scene_data", scene_data)

### world
world_list = perseus.list_world()
print(world_list)

world_instance = perseus.get_world('default')
# world_list = perseus.list_world()
# print(world_list)

# env = perseus.world.Environment()
# print("env", env)
# world_instance.set_environment(env)

world_instance.load_map(map_name)
world_instance.load_scenario(scene_name)

world_instance.start()

simulation_time = 10*1000 # 10s
t = 0
step = 10 # 10ms
while t < simulation_time:
    world_instance.step(step)
    t = t + step

world_instance.stop()


