/**
 * 
pub enum LaneType {
    #[default]
    NONE,
    CityDriving,
    SideWalk,
    Shoulder,
    Bus,
    Biking,
    Parking,
    Bidirectional,
}

/// 车道定义

#[derive(Debug, Clone)]
pub struct LaneInfo {
    pub id: i32,
    pub r#type: LaneType,

    pub central_lane_curve: LaneCurveInfo,
    pub width: f64,
}
*/

import { LineCurve } from "./line_curve";
import { perseus } from "../serialize/map";
import { LanePoint } from "./util";

export enum LaneType {
    NONE = 0,
    CityDriving = 1,
    SideWalk = 2,
    Shoulder = 3,
    Bus = 4,
    Biking = 5,
    Parking = 6,
    Bidirectional = 7
}

export class Lane {
    id!: number;
    type!: LaneType;

    central_lane_curve!: LineCurve;
    width!: number;

    constructor(id:number, type : LaneType) {
        this.id = id;
        this.type = type;
    }

    // 获取 绘制到UI上的点坐标
    get_draw_points(step : number = 0.2) : LanePoint[] {
        const left_lane_curve = this.central_lane_curve.translation(-this.width/2);
        const right_lane_curve =  this.central_lane_curve.translation(this.width/2);
        return left_lane_curve.get_draw_points().concat(right_lane_curve.get_draw_points().reverse());
    }


    to_proto(): perseus.api.v1.map.Lane {
        return perseus.api.v1.map.Lane.fromObject({
            id : this.id,
            type : this.type,
            central_line: this.central_lane_curve.to_proto(),
            width : this.width,
        });
    }

    static from_proto(o : perseus.api.v1.map.Lane) : Lane {
        var thi = new Lane(o.id, o.type);
        thi.central_lane_curve = LineCurve.from_proto(o.central_line);
        thi.width = o.width;

        return thi;
    }
}
