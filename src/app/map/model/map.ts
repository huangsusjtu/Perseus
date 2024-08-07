/**

/// 地图sd_map
#[derive(Debug)]
pub struct SDMap {
    pub header: Header,

    // 路
    pub(crate) roads: HashMap<String, Arc<RoadInfo>>,
    // 路口
    pub(crate) junctions: HashMap<String, Arc<JunctionInfo>>,

    // 非道路的作业区域
    pub(crate) clean_areas: HashMap<String, Arc<CleanArea>>,

    // 环卫的各个 站点
    pub(crate) sites: HashMap<String, Arc<SiteInfo>>,

    // 用于 查询 车道
    pub(crate) road_infos_kdtree: KDTree2d<LineSegmentBox>,
    // 十字路口查询
    pub(crate) junctions_kdtree: KDTree2d<PointBox>,

    pub(crate) next_road_id: i32,
    pub(crate) next_lane_id: i32,
}


*/
import { perseus } from "../serialize/map";
import { CleanArea } from "./clean_area";
import { Header } from "./header";
import { Junction } from "./junction";
import { Road } from "./road";
import { Site } from "./site";


export class HdMap {
    header!: Header;
    
    // 路
    roads!: Map<string, Road>;
    // 路口
    junctions!: Map<string, Junction>;
    // 非道路的作业区域
    clean_areas!: Map<string, CleanArea>;
    // 环卫的各个 站点
    sites!: Map<string, Site>;


    next_road_id!: number;
    next_lane_id!: number;
    next_junction_id!: number;
    next_area_id!: number;
    next_site_id !: number;

    constructor(name: string) {
        this.header = new Header(name);

        this.roads = new Map();
        this.junctions = new Map();
        this.clean_areas = new Map();
        this.sites = new Map();

        this.next_road_id = 0;
        this.next_lane_id = 0;
        this.next_junction_id = 0;
        this.next_area_id = 0;
        this.next_site_id = 0;
    }




    // 序列化
    to_proto(): perseus.api.v1.map.Map {
        var roads = new Map()
        this.roads.forEach((v,k)=> {
            roads.set(k, v.to_proto());
        })
        var junctions = new Map()
        this.junctions.forEach((v,k)=> {
            junctions.set(k, v.to_proto());
        })
        var clean_areas = new Map()
        this.clean_areas.forEach((v,k)=> {
            clean_areas.set(k, v.to_proto());
        })
        var sites = new Map()
        this.sites.forEach((v,k)=> {
            sites.set(k, v.to_proto());
        })

        var thi =  perseus.api.v1.map.Map.fromObject({
            header : this.header.to_proto()
        });
        thi.roads(roads);
        thi.junctions(junctions);
        thi.clean_areas(clean_areas);
        thi.sites(sites);

        return thi;
    }

    // 反序列化
    static from_proto(o : perseus.api.v1.map.Map) : HdMap {
        let thi = new HdMap(o.header.name);
        thi.header = Header.from_proto(o.header);

        var next_road_id = 0;
        var next_lane_id = 0;
        var next_junction_id = 0;
        var next_area_id = 0;
        var next_site_id = 0;

        var roads = new Map()
        o.roads().forEach((v: perseus.api.v1.map.Road,k: string) => {
            if (v.id > next_road_id) {
                next_road_id = v.id + 1;
            }
            var road = Road.from_proto(v);
            road.left_lanes.forEach((v)=>{
                if (v.id > next_lane_id) {
                    next_lane_id = v.id+1;
                }
            })
            road.right_lanes.forEach((v)=>{
                if (v.id > next_lane_id) {
                    next_lane_id = v.id+1;
                }
            })
            roads.set(k, road);
        });
        thi.roads = roads;
        thi.next_road_id = next_road_id;

        var junctions = new Map()
        o.junctions().forEach((v: perseus.api.v1.map.Junction,k: string) => {
            if (v.id > next_junction_id) {
                next_junction_id = v.id + 1;
            }
            junctions.set(k, Junction.from_proto(v));
        });
        thi.junctions = junctions;
        thi.next_junction_id = next_junction_id;

        var clean_areas = new Map()
        o.clean_areas().forEach((v: perseus.api.v1.map.CleanArea,k: string) => {
            if (v.id > next_area_id) {
                next_area_id = v.id + 1;
            }
            clean_areas.set(k, CleanArea.from_proto(v));
        });
        thi.clean_areas = clean_areas;
        thi.next_area_id = next_area_id;

        var sites = new Map()
        o.sites().forEach((v: perseus.api.v1.map.Site,k: string) => {
            if (v.id > next_site_id) {
                next_site_id = v.id + 1;
            }
            sites.set(k, Site.from_proto(v));
        });
        thi.sites = sites;
        thi.next_site_id = next_site_id;

        return thi;
    }
}



