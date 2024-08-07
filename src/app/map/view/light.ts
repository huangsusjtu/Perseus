import * as THREE from "three";
import { EventBus } from "../eventbus";

const LIGHT_Z = 100;

class SceneLight {
    scene!: THREE.Scene;
    requestPaint: Function

    light_left = new THREE.PointLight(0xffffff, 0.5, 500);
    light_right = new THREE.PointLight(0xffffff, 0.5, 500);
    light_top = new THREE.PointLight(0xffffff, 0.5, 500);
    light_bottom = new THREE.PointLight(0xffffff, 0.5, 500);

    ambientLight = new THREE.AmbientLight(0xffffff, 0.1);

    x = 0;
    y = 0;
    z = 0;

    constructor(scene: THREE.Scene, requestPaint: Function) {
        this.scene = scene;
        this.requestPaint = requestPaint;

        this.scene.add(this.ambientLight);
        this.light_left.position.set(this.x - LIGHT_Z, this.y, LIGHT_Z);
        this.light_right.position.set(this.x + LIGHT_Z, this.y, LIGHT_Z);
        this.light_top.position.set(this.x, this.y + LIGHT_Z, LIGHT_Z);
        this.light_bottom.position.set(this.x, this.y - LIGHT_Z, LIGHT_Z);

        this.light_left.lookAt(this.x, this.y, this.z);
        this.light_right.lookAt(this.x, this.y, this.z);
        this.light_top.lookAt(this.x, this.y, this.z);
        this.light_bottom.lookAt(this.x, this.y, this.z);
        this.scene.add(this.light_left);
        this.scene.add(this.light_right);
        this.scene.add(this.light_top);
        this.scene.add(this.light_bottom);



        EventBus.addListener("on_focus_position_change", this.onHeroCarPositionChange);
    }

    onHeroCarPositionChange = (x: number, y: number, z: number) => {
        this.x = x;
        this.y = y;
        this.z = z;
        this.light_left.position.set(this.x - LIGHT_Z, this.y, LIGHT_Z);
        this.light_right.position.set(this.x + LIGHT_Z, this.y, LIGHT_Z);
        this.light_top.position.set(this.x, this.y + LIGHT_Z, LIGHT_Z);
        this.light_bottom.position.set(this.x, this.y - LIGHT_Z, LIGHT_Z);

        this.light_left.lookAt(this.x, this.y, this.z);
        this.light_right.lookAt(this.x, this.y, this.z);
        this.light_top.lookAt(this.x, this.y, this.z);
        this.light_bottom.lookAt(this.x, this.y, this.z);
        this.requestPaint();
    }
}
export { SceneLight }