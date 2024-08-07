/**
    pub struct Header {
        pub major_version: u32,
        pub minor_version : u32,
        pub name: String,
        /// 生产日期
        pub date: String,
        /// 坐标系wgs84, bd09, utm,
        pub coordinate: &'static str,
    }
 */

import { perseus } from "../serialize/map";

export enum Coordinate {
    None = 0,
    BD09 = 1,
    WGS84 = 2,
}

export class Header {
    major_version!: number;
    minor_version!: number;

    name!: string;
    date!: string;
    coordinate!: Coordinate;


    constructor(name: string) {
        this.name = name;
        this.major_version = 1;
        this.minor_version = 0;

        this.date = new Date().toISOString();
        this.coordinate = Coordinate.None;
    }


    to_proto(): perseus.api.v1.map.Header {
        return perseus.api.v1.map.Header.fromObject(
            {
                major_version: this.major_version,
                minor_version: this.minor_version,
                name: this.name,
                date: this.date,
                coordinate: this.coordinate,

            }
        );
    }

    static from_proto(o: perseus.api.v1.map.Header): Header {
        var thi = new Header(o.name);
        thi.major_version = o.major_version;
        thi.minor_version = o.minor_version;
        thi.date = o.date;
        thi.minor_version = o.minor_version;
        thi.coordinate = o.coordinate;

        return thi;
    }
}






