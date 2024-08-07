import { perseus } from "../serialize/map";

export class LanePoint {
    x!: number;
    y!: number;


    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    distance(o : LanePoint) : number {
        return Math.hypot(o.x-this.x, o.y-this.y);
    }

    to_proto(): perseus.api.v1.map.LanePoint {
        return perseus.api.v1.map.LanePoint.fromObject(
            {
                x: this.x,
                y: this.y,
                z: 0
            }
        );
    }

    static from_proto(o : perseus.api.v1.map.LanePoint) : LanePoint {
        return new LanePoint(o.x, o.y);
    }
}

export var K_MATH_EPSILON = 1e-6;

export function cross_prod(x0: number, y0: number, x1: number, y1: number) : number{
    return x0 * y1 - x1 * y0;
}

export function inner_prod(x0: number, y0: number, x1: number, y1: number) : number{
    return x0 * x1 + y0 * y1;
}

export function is_straight_line(x0: number, y0: number, x1: number, y1: number) : boolean {
    if ((x0 == 0.0 && y0 == 0.0) || (x1 == 0.0 && y1 == 0.0)) {
        return true;
    }
    let ret = (x0 * x1 + y0 * y1) / (Math.sqrt(x0 * x0 + y0 * y0) * Math.sqrt(x1 * x1 + y1 * y1));

    return ret > 0.9999;
}



export function normalize_angle(angle: number) : number {
    var a = (angle + Math.PI) % (2.0 * Math.PI);
    if (a < 0.0) {
        a += 2.0 * Math.PI;
    }
    return a - Math.PI;
}

export function binary_search(host: number[], target: number) : number {
    var size = host.length;
    var left = 0;
    var right = size - 1;
    while (left < right) {
        let mid = left + size / 2;
        let t = host[mid];
        if (t < target) {
            left = mid + 1;
        } else if (t > target) {
            right = mid;
        } else {
            return mid;
        }

        size = right - left;
    }

    if (left == host.length) {
        left -= 1;
    }
    return left;
}


export function to_2pi(a: number) : number {
    while (a > 2.0 * Math.PI) {
        a = a - 2.0 * Math.PI;
    }
    while (a < 0.0) {
        a = a + 2.0 * Math.PI;
    }
    return a;
}

/// 返回两个角度 中间值
export function middle_of_two_radian(a: number, b: number) : number {
    let max =  Math.max(a, b);
    let min = Math.min(a, b);
    if (max - min < Math.PI) {
        return min + (max - min) / 2.0;
    } else {
        let t = (min + max) / 2.0;
         if (t > Math.PI) {
            return (min + max) / 2.0 - Math.PI
        } else {
            return (min + max) / 2.0 + Math.PI
        };
    }
}
/// 返回两个角度 夹角
export function radian_of_two_radian(a: number, b: number) : number {
    let delta = Math.abs(a - b);
    return Math.min(delta, (2.0 * Math.PI - delta));
}

