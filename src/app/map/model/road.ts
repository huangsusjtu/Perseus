/**
/// 道路定义
#[derive(Debug)]
pub struct RoadInfo {
    pub id: String, // 唯一ID， 命名规范 r-1
    pub name: String,
    pub length: f64,

    pub r#type: RoadType,

    pub center_line: LaneCurveInfo,

    pub left_lanes: Vec<LaneInfo>,
    pub right_lanes: Vec<LaneInfo>,

    // 路的上下游
    pub link: Option<Vec<RoadLink>>,
    //  todo!() add more
    // 道路两侧 树木类型， 密度，等等
}

 */
import { perseus } from "../serialize/map";
import { Lane } from "./lane";
import { LineCurve } from "./line_curve";



export enum RoadType {
    None = 0,
    Major = 1,
    Minor = 2,
    SideWalk = 3
}

export class RoadLink {
    link_type!: string;   // predecessor or successor
    element_type!: string; // junction or road
    element_id!: number; // junction or road id


    constructor(link_type:string, element_type:string, element_id:number) {
        this.link_type = link_type;
        this.element_type = element_type;
        this.element_id = element_id;
    }

    to_proto(): perseus.api.v1.map.Road.RoadLink {
        return perseus.api.v1.map.Road.RoadLink.fromObject(
            {
                link_type: this.link_type,
                element_type: this.element_type,
                element_id: this.element_id,
            }
        );
    }
    static from_proto(o : perseus.api.v1.map.Road.RoadLink) : RoadLink {
        return new RoadLink(o.link_type, o.element_type, o.element_id);
    }
}

export class Road {
    id!: number;
    name!: string;

    length!: number;
    type!: RoadType;
    center_line!: LineCurve;
    left_lanes!: Lane[];
    right_lanes!: Lane[];
    link!: RoadLink[];


    constructor(id: number) {
        this.id = id;
    }

    to_proto(): perseus.api.v1.map.Road {
        var left_lanes = new Array();
        this.left_lanes.forEach((i)=> {
            left_lanes.push(i.to_proto())
        });
        var right_lanes = new Array();
        this.right_lanes.forEach((i)=> {
            right_lanes.push(i.to_proto())
        });
        var link = new Array();
        this.link.forEach((i)=> {
            link.push(i.to_proto())
        })

        return perseus.api.v1.map.Road.fromObject({
            id : this.id,
            name : this.name,
            length : this.length,
            type : this.type,
            central_line : this.center_line.to_proto(), 
            left_lanes : left_lanes, 
            right_lanes : right_lanes,
            link: link
        })
    }

    static from_proto(o : perseus.api.v1.map.Road) : Road {
        var thi = new Road(o.id);
        thi.name = o.name;
        thi.length = o.length;
        thi.type = o.type;
        thi.center_line = LineCurve.from_proto(o.central_line);
        var left_lanes = new Array();
        o.left_lanes.forEach((i)=> {
            left_lanes.push(Lane.from_proto(i));
        });
        var right_lanes = new Array();
        o.right_lanes.forEach((i)=> {
            right_lanes.push(Lane.from_proto(i));
        });
        var link = new Array();
        o.link.forEach((i)=> {
            link.push(RoadLink.from_proto(i))
        })

        thi.left_lanes = left_lanes;
        thi.right_lanes = right_lanes;
        thi.link = link;
        return thi;
    }
}
