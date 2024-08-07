/**

pub enum SiteType {
    CleanSite,       // 清洗点
    ChargingSite,    // 充电站
    ParkingSite,     // 停车点
    TransferSite,    // 中转站
    WaterSupplySite, // 加水站
    MaintenanceSite, // 维修站
    GasSite,         // 加油站
}

pub struct SiteInfo {
    pub id: String, // 唯一ID， 命名规范 s-1
    pub r#type: SiteType,
    pub name: String,
    pub position: LanePoint,
}
 */
import { perseus } from "../serialize/map";
import { LanePoint } from "./util";


export enum SiteType {
    None = 0,
    CleanSite = 1,     // 清洗点
    ChargingSite = 2,    // 充电站
    ParkingSite = 3,     // 停车点
    TransferSite = 4,    // 中转站
    WaterSupplySite = 5, // 加水站
    MaintenanceSite = 6, // 维修站
    GasSite = 7,         // 加油站
}


export class Site { 
    id!: number; // 唯一ID， 命名规范 1
    name!: string;
    type!: SiteType;
    position!: LanePoint;


    constructor(id: number) {
        this.id = id;
    }


    to_proto(): perseus.api.v1.map.Site {
        return perseus.api.v1.map.Site.fromObject({
            id : this.id,
            name : this.name,
            type : this.type,
            position : this.position.to_proto(),
        })
    }

    static from_proto(o : perseus.api.v1.map.Site) : Site {
        var thi = new Site(o.id);
        thi.name = o.name;
        thi.type = o.type; 
        thi.position = LanePoint.from_proto(o.position);
        return thi;
    }

}


