/**
pub struct JunctionInfo {
    pub id: String, // object id
    pub name: String,
    pub center: LanePoint,        // 坐标
    pub boundary: Vec<LanePoint>, // 区域

    pub road_connections: Vec<ConnectionInfo>, // 哪些road连接起来了
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: String,
    pub road_in: String,
    pub road_out: String,
}

*/

import { LanePoint } from "./util";
import { perseus } from "../serialize/map";

export class Junction {
    id!: number;
    name!: string;

    position!: LanePoint;
    polygon?: LanePoint[];
    road_connections?: ConnectionInfo[];


    constructor(id: number, position: LanePoint) {
        this.id = id;
        this.name = "Juntion-" + id;
        this.position = position;
    }


    to_proto(): perseus.api.v1.map.Junction {
        var polygon = new Array();
        this.polygon?.forEach((i)=>{
            polygon.push(i.to_proto())
        })

        var road_connections= new Array();
        this.road_connections?.forEach((i)=>{
            road_connections.push(i.to_proto())
        })

        return perseus.api.v1.map.Junction.fromObject({
            id : this.id,
            name : this.name,
            position : this.position.to_proto(),
            polygon :polygon ,
            connection: road_connections,

        });
    }

    static from_proto(o : perseus.api.v1.map.Junction) : Junction {
        var thi = new Junction(o.id, LanePoint.from_proto(o.position));

        var polygon = new Array();
        o.polygon?.forEach((i)=>{
            polygon.push(LanePoint.from_proto(i))
        })

        var road_connections = new Array();
        o.connection?.forEach((i)=>{
            road_connections.push(ConnectionInfo.from_proto(i))
        })

        thi.polygon = polygon;
        thi.road_connections = road_connections;


        return thi;
    }
}

export class ConnectionInfo {
    id!: number;
    road_in!: string;
    road_out!: string;

    constructor(id: number, road_in: string, road_out: string) {
        this.id = id;
        this.road_in = road_in;
        this.road_out = road_out;
    }

    to_proto(): perseus.api.v1.map.Junction.ConnectionInfo {

        return perseus.api.v1.map.Junction.ConnectionInfo.fromObject({
            id : this.id,
            road_in: this.road_in,
            road_out: this.road_out,
        });
    }

    static from_proto(o : perseus.api.v1.map.Junction.ConnectionInfo) : ConnectionInfo {
        return new ConnectionInfo(o.id,o.road_in,o.road_out);    
    }
}


