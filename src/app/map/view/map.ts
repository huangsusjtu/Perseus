

import { EventBus } from '../eventbus'
import * as THREE from "three";
import { Group } from 'three';
import { drawMapLane } from './map_util';
import { HdMap } from '../model/map';

let instance_id = 0;

class HdMapRender {
    requestPaint: Function

    hdmap?: HdMap;

    // 所有地图元素的根
    map_mesh: Group;
    // 所有的车道中心线
    center_line_mesh: Group;
    // 车道面
    lane_mesh: Group;

    instance_id: number;

    constructor(scene: THREE.Scene, requestPaint: Function) {
        this.requestPaint = requestPaint;

        EventBus.addListener("on_map_elements", this.onMapElements);


        this.map_mesh = new Group();
        this.center_line_mesh = new Group();
        this.lane_mesh = new Group();

        this.instance_id = instance_id++;


        this.map_mesh.add(this.center_line_mesh)
        this.map_mesh.add(this.lane_mesh)
        scene.add(this.map_mesh)
    }


    public onMapElements = (hdmap: HdMap) => {
        console.log("MapRender: ", hdmap);

        let has_new_element: boolean = false;
        if (hdmap.roads.size > 0) {
            hdmap.roads.forEach((v,k)=> {
                for (const lane of v.left_lanes) {
                    drawMapLane(this.lane_mesh, this.center_line_mesh, lane);
                }
                for (const lane of v.right_lanes) {
                    drawMapLane(this.lane_mesh, this.center_line_mesh, lane);
                }

            })
            has_new_element = true;
        }




        if (has_new_element) {
            this.requestPaint();
        }
    }
}


export { HdMapRender };
