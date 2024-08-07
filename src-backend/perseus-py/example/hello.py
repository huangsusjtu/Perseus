import random

import perseus

perseus.init("/home/huangsu/work/Perseus/src-backend/libformat/asset/opendrive", "/home/huangsu/work/Perseus/src-backend/libmodel/asset/sanitation")


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


