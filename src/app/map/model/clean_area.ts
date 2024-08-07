import { perseus } from "../serialize/map";
import { LanePoint } from "./util";

/**
    pub struct CleanArea {
        pub name: String,

        // 位置
        pub position: LanePoint,
        pub polygon: Option<Vec<LanePoint>>,
    }
*/

export class CleanArea {
    id!: number;
    name!: string;
    position!: LanePoint;

    polygon?: LanePoint[];

    constructor() {

    }


    to_proto(): perseus.api.v1.map.CleanArea {
        var polygon = new Array();
        this.polygon?.forEach((i) => {
            polygon.push(i.to_proto())
        });

        return perseus.api.v1.map.CleanArea.fromObject(
            {
                id: this.id,
                name:this.name,
                position: this.position.to_proto(),
                polygon: polygon,
            }
        );
    }

    static from_proto(o : perseus.api.v1.map.CleanArea) : CleanArea {
        var thi = new CleanArea();
        thi.id = o.id;
        thi.name = o.name;
        thi.position = LanePoint.from_proto(o.position);
        var polygon = new Array();
        o.polygon.forEach((i)=> {
            polygon.push(LanePoint.from_proto(i));
        })

        return thi;
    }
}


