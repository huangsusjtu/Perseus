/**
pub struct LaneCurveInfo {
    pub segments: Vec<GeometryLine>,
    pub segments_length: Vec<f64>,
    pub length: f64,
}

 */

import { binary_search, cross_prod, inner_prod, LanePoint, radian_of_two_radian, to_2pi } from "./util";
import { perseus } from "../serialize/map";


const K_MATH_EPSILON = 0.01;

export class LineCurve {
    segments!: GeometryLine[];
    segments_length!: number[];
    length!: number;

    constructor(lines : GeometryLine[]) {
        this.segments = lines;

        var segments_length = new Array();
        var length = 0;
        lines.forEach((e)=>{
            length += e.length;
            segments_length.push(length)
        })
        this.segments_length = segments_length;
        this.length = length;
    }

    // 获取 绘制到UI上的点坐标
    get_draw_points(step : number = 0.2) : LanePoint[] {
        var ret = new Array();

        for (var i = 0; i < this.segments.length; ++i) {
            this.segments[i].get_smooth_range_points(0, this.length, 0.1);
        }
        return ret;
    }

    get_heading(s: number) : number {
        var idx = binary_search(this.segments_length, s);
        return this.segments[idx].get_heading(s);
    }

    get_smooth_point(s: number)  : LanePoint {
        var idx = binary_search(this.segments_length, s);
        return this.segments[idx].get_smooth_point(s);
    }

    get_smooth_range_points(s: number, e: number, step: number) : LanePoint[] {
        var ret = new Array();
        var idx1 = binary_search(this.segments_length, s);
        var idx2 = binary_search(this.segments_length, e);
        for (var i = idx1; i <= idx2; i++) {
            ret = ret.concat(this.segments[i].get_smooth_range_points(s, e, 0.1));
        }
        return ret;
    }

    get_projection(x: number, y: number) : {s : number, l: number}  {
        if (this.segments.length == 0) {
            throw new Error("segnment empty, get_projection");
        }
        var min_dist = Number.MAX_VALUE;
        var seg_num = this.segments.length;
        var min_index = 0;
        var i = 0;
        while (i < seg_num) {
            var {dist, point}  = this.segments[i].distance(x, y);
            if (dist < min_dist) {
                min_index = i;
                min_dist = dist;
            }
            i += 1;
        }

        var lateral;
        var accumulate_s = 0.0;
        for (var i = 0; i < min_index; ++i) {
            accumulate_s += this.segments[i].length;
        }

        var nearest_seg = this.segments[min_index];
        var {s, l} = nearest_seg.get_projection(x, y);
        accumulate_s += Math.min(s, nearest_seg.length);
        if (min_index == 0) {
            accumulate_s = Math.min(s,nearest_seg.length);
            if (s < 0) {
                lateral = l;
            } else {
                if (l > 0) {
                    lateral = min_dist;
                } else {
                    lateral = -min_dist;
                }
            }
        } else if (min_index == seg_num - 1) {
            accumulate_s = accumulate_s + Math.max(0, s);
            if (s > 0) {
                lateral = l;
            } else {
                if (l > 0) {
                    lateral = min_dist;
                } else {
                    lateral = -min_dist;
                }
            }
        } else {
            accumulate_s =
                accumulate_s + Math.max(Math.min(s, nearest_seg.length), 0.0);
            if (l > 0) {
                lateral = min_dist;
            } else {
                lateral = -min_dist;
            }
        }

        return {s : accumulate_s, l : lateral};
    }
    
    distance(x: number, y: number) : number {
        if (this.segments.length == 0) {
            throw new Error("segnment empty, get_projection");
        }
        var min_dist = Number.MAX_VALUE;
        var seg_num = this.segments.length;
        var i = 0;
        while (i < seg_num) {
            var {dist, point}  = this.segments[i].distance(x, y);
            if (dist < min_dist) {
                min_dist = dist;
            }
            i += 1;
        }
        return min_dist;
    }

    translation(offset: number) : LineCurve {
        var segments = new Array();
        this.segments.forEach((seg) => {
            segments.push(seg.translation(offset))
        });
        return new LineCurve(segments);
    }



    to_proto(): perseus.api.v1.map.LineCurve {
        var segments = new Array();
        this.segments.forEach((i)=> {
            if (i instanceof StraightLine) {
                segments.push(i.to_proto());
            } else if ( i instanceof ArcLine) {
                segments.push(i.to_proto());
            }
        });

        return perseus.api.v1.map.LineCurve.fromObject({
            segments : segments,
            length : this.length,
        })

    }
    static from_proto(o : perseus.api.v1.map.LineCurve) : LineCurve {
        var segments = new Array();
        o.segments.forEach((i)=> {
            if (i instanceof perseus.api.v1.map.LineCurve.StraightLine) {
                segments.push(StraightLine.from_proto(i))
            } else if (i instanceof perseus.api.v1.map.LineCurve.ArcLine) {
                segments.push(ArcLine.from_proto(i))
            }
        });

        return new LineCurve(segments);
    }


    static from_points(points : LanePoint[])  : LineCurve {
        var lines : GeometryLine[] = new Array();

        var start_point = points[0];
        var s = 0.0;
        for (var i = 1; i < points.length - 1; i++ ) {
            let hdg1 = Math.atan2(points[i - 1].y - points[i].y, points[i - 1].x - points[i].x);
            let len1 = Math.hypot(points[i - 1].y - points[i].y, points[i - 1].x - points[i].x);
            let hdg2 = Math.atan2(points[i + 1].y - points[i].y, points[i + 1].x - points[i].x);
            let len2 = Math.hypot(points[i + 1].y - points[i].y, points[i + 1].x - points[i].x);

            if (Math.abs(hdg1 + hdg2 - 2.0 * Math.PI) < 0.01) {
                // 一条直线， 所以中间这个点就忽略掉了
            } else {
                // 对于 三个点不在一条直线的情况，  前面有一条直线， 中间是
                // 圆弧线， 后续的情况根据后续的点决定
                // 1. 直线
                let trip_len = Math.min(
                    (i==1?len1:len1 / 2.0), 
                    (i==points.length - 2?len2:len2/2)
                );
                let straight_line_end = new LanePoint(
                    points[i].x + trip_len * Math.cos(hdg1),
                    points[i].y + trip_len * Math.sin(hdg1),
                );
                let straight_line_len =
                    start_point.distance(straight_line_end);
                if (straight_line_len > K_MATH_EPSILON) {
                    let line = new StraightLine(start_point.x, start_point.y, to_2pi(hdg1 + Math.PI), straight_line_len, s);
                    s += straight_line_len;
                    lines.push(line);
                    start_point = straight_line_end;
                }
                // 2. 圆弧线
                let jiajiao = radian_of_two_radian(hdg1, hdg2);
                let r = trip_len * Math.tan(jiajiao / 2.0); // 半径
                let len_of_radian = r * (Math.PI - jiajiao);
                let radian = new ArcLine(start_point.x, start_point.y, to_2pi(hdg1 + Math.PI), len_of_radian, 1.0 / r,s);
                start_point = new LanePoint(
                    points[i].x + trip_len * Math.cos(hdg2),
                    points[i].y + trip_len * Math.sin(hdg2),
                );
                lines.push(radian);
                s += len_of_radian;
            }
        }
        // 3. 最后一个圆弧的端点 到 最后一个点 之间是直线
        let end_point = points[points.length - 1];
        let straight_line_len = start_point.distance(end_point);
        if (straight_line_len > K_MATH_EPSILON) {
            let line = new StraightLine(start_point.x, start_point.y, 
                Math.atan2(end_point.y - start_point.y,end_point.x - start_point.x),
                straight_line_len, s
                );
            lines.push(line);
        }

        return new LineCurve(lines);
    }
}

interface GeometryLine {
    hdg: number;
    length: number;
    s: number;
    x: number;
    y: number;

    get_heading: (s: number) => number
    get_smooth_point: (s: number) => LanePoint
    get_smooth_range_points: (s: number, e: number, step: number) => LanePoint[]
    get_projection: (x: number, y: number) => {s : number, l : number}
    distance:(x: number, y: number) => {dist:number,  point: LanePoint}
    translation: (offset: number) => GeometryLine
}


class StraightLine implements GeometryLine {
    hdg: number;
    length: number;
    s: number;
    x: number;
    y: number;

    constructor(x: number, y: number, hdg: number, length: number, s: number = 0.0) {
        this.hdg = hdg;
        this.length = length;
        this.s = s;
        this.x = x;
        this.y = y;
    }

    get_heading(s: number) {
        return this.hdg;
    }

    get_s(): number {
        return this.s;
    }

    get_length(): number {
        return this.length;
    }

    get_smooth_point(s: number): LanePoint {
        if (s < this.s || s > this.s + this.length) {
            throw new Error("get_smooth_point: s error");
        }
        return new LanePoint(this.x + (s - this.s) * Math.cos(this.hdg),
            this.y + (s - this.s) * Math.sin(this.hdg));
    }

    get_smooth_range_points(s: number, e: number, step : number) : LanePoint[] {
        s = Math.max(s, this.s);
        e = Math.min(e, this.s+this.length);
        if (s > e) {
            throw new Error("get_smooth_point: s or e error");
        }

        return [new LanePoint(this.x + (s - this.s) * Math.cos(this.hdg), this.y + (s - this.s) * Math.sin(this.hdg)), 
            new LanePoint(this.x + (e - this.s) * Math.cos(this.hdg), this.y + (e - this.s) * Math.sin(this.hdg))];
    }

    get_projection(x: number, y: number) : {s : number, l : number} {
        var sin = Math.sin(this.hdg);
        var cos = Math.cos(this.hdg);
        let x0 = x - this.x;
        let y0 = y - this.y;
        let proj = inner_prod(cos, sin, x0, y0);
        let prod = cross_prod(cos, sin, x0, y0);

        return {s : proj, l : prod};

    }

    distance(x: number, y: number) : {dist:number,  point: LanePoint}{
        var sin = Math.sin(this.hdg);
        var cos = Math.cos(this.hdg);
        let x0 = x - this.x;
        let y0 = y - this.y;
        let proj = x0 * cos + y0 * sin;
        if (proj < 0.0) {
            return {
                dist : Math.sqrt(x0*x0 + y0*y0),  point: new LanePoint(this.x, this.y)};
        } else if (proj > this.length) {
            return {
                dist : Math.sqrt(x0*x0 + y0*y0), point: new LanePoint(this.x, this.y)};
        }

        return {
            dist : Math.abs(x0 * sin - y0 * cos),
            point : new LanePoint(this.x + cos * proj, this.y + sin * proj)
        };
    }

    translation(offset: number) {
        var signum = Math.sign(offset);
        var sin = Math.sin(this.hdg - signum * Math.PI / 2.0);
        var cos = Math.cos(this.hdg - signum * Math.PI / 2.0);

        let n_x = this.x + cos * Math.abs(offset);
        let n_y = this.y + sin * Math.abs(offset);
        return new StraightLine (
            n_x,
            n_y,
            this.hdg,
            this.length,
            this.s,
        );
    }


    to_proto(): perseus.api.v1.map.LineCurve.StraightLine {
        return perseus.api.v1.map.LineCurve.StraightLine.fromObject({
            hdg : this.hdg,
            length : this.length,
            start_s: this.s,
            position : new LanePoint(this.x, this.y).to_proto()
        })
    }
    static from_proto(o : perseus.api.v1.map.LineCurve.StraightLine) : StraightLine {
        return new StraightLine(o.position.x, o.position.y,
            o.hdg, o.length, o.start_s);
    }
}



class ArcLine implements GeometryLine {
    hdg: number;
    length: number;
    s: number;
    x: number;
    y: number;
    curvature!: number;

    constructor(x: number, y: number, hdg: number, length: number, curvature : number, s: number = 0.0) {
        this.hdg = hdg;
        this.length = length;
        this.s = s;
        this.x = x;
        this.y = y;
        this.curvature = curvature;
    }

    base_info() : number[] {
        let r = 1.0 / Math.abs(this.curvature); // 半径
        let signum = Math.sign(this.curvature);
        let to_center_direction = this.hdg + signum * Math.PI / 2.0; // 圆弧起点往圆心的方向
                                                                                 // 圆心坐标
        let center_x = Math.cos(to_center_direction) * r + this.x;
        let center_y = Math.sin(to_center_direction) * r + this.y;

        let start_arc = Math.atan2(this.y - center_y, this.x - center_x); // 圆弧起点和X轴夹角
        let end_arc =
            to_2pi(start_arc + signum * this.length / r); // 圆弧终点 和 圆弧起点 之间弧度夹角
        return [
            r,
            to_center_direction,
            center_x,
            center_y,
            start_arc,
            end_arc,
        ];
    }

    get_heading(s: number) : number {
        var [r, _to_center_direction, center_x, center_y, _start_arc, _end_arc] = this.base_info();
        let signum = Math.sign(this.curvature);
        let arc = (s - this.s) / r; // 目标点 和 起点 之间弧度夹角
        let target_arc = Math.atan2(this.y - center_y, this.x - center_x);
            + signum * arc;
        return to_2pi(target_arc + signum * Math.PI / 2.0);
    }

    get_s(): number {
        return this.s;
    }

    get_length(): number {
        return this.length;
    }
    
    get_smooth_point(s: number) : LanePoint{
        if (s < this.s
        || s > this.s + this.length)
        {
            throw new Error("StraightLine:get_smooth_point");
        }
        let [r, _to_center_direction, center_x, center_y, _start_arc, _end_arc] = this.base_info();

        let arc = (s - this.s) / r; // 目标点 和 起点 之间弧度夹角
        let target_arc = Math.atan2(this.y - center_y,this.x - center_x) + Math.sign(this.curvature) * arc;

        return new LanePoint(
            center_x + r * Math.cos(target_arc),
            center_y + r * Math.sin(target_arc),
        );
    }

    get_smooth_range_points(s: number, e: number, step : number = 0.1) : LanePoint[]
     {
        s = Math.max(s, this.s);
        e = Math.min(e, this.s+this.length);

        let [r, to_center_direction, center_x, center_y, base_arc, _e] = this.base_info();
        let signum = Math.sign(this.curvature);
        let start_arc = base_arc + signum * (Math.max(s, this.s) - this.s) / r; // 目标起点和X轴夹角
        let end_arc = base_arc + signum * (Math.min(e, this.s + this.length) - this.s) / r; // 目标终点和X轴夹角

        var ret = new Array();
        if (signum > 0) {
            var target_arc = start_arc;
            if (end_arc < start_arc) {
                end_arc = end_arc + 2.0 * Math.PI
            }
            while (target_arc < end_arc) {
                ret.push(new LanePoint(
                    center_x + r * Math.cos(target_arc),
                    center_y + r * Math.sin(target_arc),
                ));
                target_arc = target_arc + step / r;
            }
        } else {
            var target_arc = start_arc;
            if (end_arc > start_arc) {
                end_arc = end_arc - 2.0 * Math.PI
            }
            while (target_arc > end_arc) {
                ret.push(new LanePoint(
                    center_x + r * Math.cos(target_arc),
                    center_y + r * Math.sin(target_arc),
                ));
                target_arc = target_arc - step / r;
            }
        }
        return ret;
    }

    get_projection(x: number, y: number) : {s : number, l : number } {
        let [r, _to_center_direction, center_x, center_y, start_arc, end_arc] =
            this.base_info();
        let target_arc = Math.atan2(y - center_y, x - center_x);

        var s = 0.0;
         if (this.curvature > 0) {
            if (end_arc > start_arc) {
                s = target_arc - start_arc
            } else {
                if (target_arc > start_arc) {
                    s = target_arc - start_arc;
                } else {
                    s = target_arc - start_arc + 2.0 * Math.PI;
                }
            }
        } else {
            // 顺时针
            if (start_arc > end_arc) {
              s =  -(target_arc - start_arc)
            } else {
                if (target_arc < start_arc) {
                   s= -(target_arc - start_arc)
                } else {
                   s = (target_arc - start_arc - 2.0 * Math.PI)
                }
            }
        };
        s = s * r;
        let l = Math.hypot(center_x + r * Math.cos(target_arc) - x, center_y + r * Math.sin(target_arc) - y);
        return {s : s, l : l};
    }

    distance(x: number, y: number):  {dist:number,  point: LanePoint} {
        let [r, _to_center_direction, center_x, center_y, start_arc, end_arc] =
            this.base_info();
        let target_arc = Math.atan2(y - center_y, x - center_x);

        var s = 0.0;
        if (this.curvature > 0) {
            if (end_arc > start_arc) {
                s = target_arc - start_arc;
            } else {
                if (target_arc > start_arc) {
                   s = target_arc - start_arc;
                } else {
                    s = target_arc - start_arc + 2.0 * Math.PI;
                }
            }
        } else {
            // 顺时针
            if (start_arc > end_arc) {
                -(target_arc - start_arc)
            } else {
                if (target_arc < start_arc) {
                    s =  -(target_arc - start_arc)
                } else {
                    s = (target_arc - start_arc - 2.0 * Math.PI)
                }
            }
        };
        s = s * r;
        let l = Math.hypot(center_x + r * Math.cos(target_arc) - x, center_y + r * Math.sin(target_arc) - y);
        return {
            dist : l,
            point : new LanePoint(
                center_x + r * Math.cos(target_arc),
                center_y + r * Math.sin(target_arc),
        
        )};
    }

    translation(offset: number) {
        let [r, to_center_direction, center_x, center_y, start_arc, end_arc] = this.base_info();
        let signum = Math.sign(this.curvature);
        let new_r = r + signum * offset; // 新半径

        return new ArcLine(
            center_x + new_r * Math.cos(start_arc),
            center_y + new_r * Math.sin(start_arc),
            this.hdg,
            this.length * new_r / r,
            this.curvature,
            this.s,
        );
    }



    to_proto(): perseus.api.v1.map.LineCurve.ArcLine {
        return perseus.api.v1.map.LineCurve.ArcLine.fromObject({
            hdg : this.hdg,
            length : this.length,
            start_s: this.s,
            position : new LanePoint(this.x, this.y).to_proto(),
            curvature : this.curvature,
        })
    }
    static from_proto(o : perseus.api.v1.map.LineCurve.ArcLine) : ArcLine {
        return new ArcLine(o.position.x, o.position.y,
            o.hdg, o.length, o.curvature, o.start_s);
    }
}







