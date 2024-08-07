import * as THREE from "three";
import { Junction } from "../model/junction";
import { Lane, LaneType } from "../model/lane";
import { LineCurve } from "../model/line_curve";
import * as Theme from "./theme";

const white_real_line_material = new THREE.LineBasicMaterial({ color: Theme.line_white_color });
const white_dash_line_material = new THREE.LineDashedMaterial({
    color: Theme.line_white_color,//线段的颜色
    dashSize: 0.2,//短划线的大小
    gapSize: 0.2//短划线之间的距离
})

const crossroad_material = new THREE.MeshBasicMaterial({ color: Theme.crossroad_color });
const crosswalk_material = new THREE.MeshBasicMaterial({ color: Theme.crosswalk_color });
const ramp_material = new THREE.MeshBasicMaterial({ color: Theme.ramp_color });
const object_material = new THREE.MeshBasicMaterial({ color: Theme.object_color });
const curb_material = new THREE.MeshBasicMaterial({ color: Theme.curb_color });
const strip_material = new THREE.MeshBasicMaterial({ color: Theme.strip_color });

const lane_unknown_material = new THREE.MeshBasicMaterial({ color: Theme.lane_unknown_color });
const lane_city_dirving_material = new THREE.MeshBasicMaterial({ color: Theme.lane_city_dirving_color });
const lane_biking_material = new THREE.MeshBasicMaterial({ color: Theme.lane_biking_color });
const lane_side_walk_material = new THREE.MeshBasicMaterial({ color: Theme.lane_side_walk_color });
const lane_waiting_area_material = new THREE.MeshBasicMaterial({ color: Theme.lane_waiting_area_color });
const lane_hybrid_material = new THREE.MeshBasicMaterial({ color: Theme.lane_hybrid_color });
const lane_parking_material = new THREE.MeshBasicMaterial({ color: Theme.lane_parking_color });
const lane_emergency_material = new THREE.MeshBasicMaterial({ color: Theme.lane_emergency_color });






/* 画地图上的线*/
function drawMapLineCurve(line_group: THREE.Group, line_curve: LineCurve) {
    // 车道线
    const line_z = 0.001
    let points: THREE.Vector3[] = [];
    line_curve.get_draw_points().forEach(e => {
        points.push(new THREE.Vector3(e.x, e.y, line_z));
    });


    const line = new THREE.Line(new THREE.BufferGeometry().setFromPoints(points), white_dash_line_material);
    line.computeLineDistances();
    line_group.add(line);
    // const line = new THREE.Line(new THREE.BufferGeometry().setFromPoints(points), white_real_line_material);
    // line_group.add(line);
}

/* 画地图车道面*/
function drawMapLane(lane_mesh: THREE.Group, line_group: THREE.Group, lane: Lane) {
    const laneshape = new THREE.Shape()
    var i = 0;
    let lane_points = lane.get_draw_points();
    laneshape.moveTo(lane_points[0].x, lane_points[0].y)
    for (i = 1; i < lane_points.length; i++) {
        laneshape.lineTo(lane_points[i].x, lane_points[i].y)
    }

    const geometry = new THREE.ShapeGeometry(laneshape);
    var material = lane_unknown_material
    switch (lane.type) {
        case LaneType.NONE: {
            material = lane_unknown_material
            break
        }
        case LaneType.CityDriving: {
            material = lane_city_dirving_material
            break
        }
        case LaneType.Biking: {
            material = lane_biking_material
            break
        }
        case LaneType.Parking: {
            material = lane_parking_material
            break
        }
        case LaneType.Shoulder: {
            material = lane_emergency_material
            break
        }
        case LaneType.SideWalk: {
            material = lane_side_walk_material
            break
        }
        case LaneType.Bus: {
            material = lane_waiting_area_material
            break
        }
        case LaneType.Bidirectional: {
            material = lane_hybrid_material
            break
        }
    }
    const mesh = new THREE.Mesh(geometry, material);
    lane_mesh.add(mesh)

    if (lane.type == LaneType.CityDriving) {
        drawMapLineCurve(line_group, lane.central_lane_curve);
    }
}


function drawJunction(crosswalk_mesh: THREE.Group, junction: Junction) {
    // const crosswalkshape = new THREE.Shape()
    // var i = 0;
    // crosswalkshape.moveTo(junction.polygon[0].x, junction.polygon[0].y)
    // for (i = 1; i < crosswalk.boundary.length; i++) {
    //     crosswalkshape.lineTo(crosswalk.boundary[i].x, crosswalk.boundary[i].y)
    // }
    // const geometry = new THREE.ShapeGeometry(crosswalkshape);
    // const mesh = new THREE.Mesh(geometry, crosswalk_material);
    // crosswalk_mesh.add(mesh)
}


export {  drawMapLineCurve, drawMapLane }

















export { };