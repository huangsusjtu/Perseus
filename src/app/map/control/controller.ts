
import React, { useEffect, useRef } from "react";
import * as THREE from "three";
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { SceneLight } from "../view/light";

let scale = 10.0
let zoom = scale
const CAMERA_Z = 100;
const minZoom = 0.01 // 缩小
const maxZoom = 20 // 放大
const dollyScale = 1.3

class ThreedContext {
    scene: THREE.Scene;
    light!: SceneLight;
    renderer: THREE.WebGLRenderer;
    control: any = null;
    camera2d: any = null;

    target_x: number = 0; // camera 指向的地图坐标
    target_y: number = 0;
    // 处理 拖动
    mousePressed = false;
    inited = false;

    // 编辑模式
    editing = false;


    markDirty = () => {
        this.renderer.render(this.scene, this.camera2d);
    }
    constructor() {

    }

    init() {
        if (!this.inited) {
            this.inited = true;
            this.scene = new THREE.Scene();
            this.light = new SceneLight(this.scene, this.markDirty);
            this.renderer = new THREE.WebGLRenderer({ antialias: true });
            this.renderer.setClearColor(0x667388, 1.0); // 0x667388
            this.renderer.setSize(window.innerWidth, window.innerHeight);

            this.camera2d = new THREE.OrthographicCamera(
                -window.innerWidth / (scale * 2),
                window.innerWidth / (scale * 2),
                window.innerHeight / (scale * 2),
                -window.innerHeight / (scale * 2),
                1, +
            1000)
            this.camera2d.position.set(0, 0, 0);
            this.camera2d.lookAt(new THREE.Vector3(0, 0, 0));
            this.control = new OrbitControls(this.camera2d, this.renderer.domElement);

            this.renderer.domElement.addEventListener('pointerup', this.onPointerUp);
            this.renderer.domElement.addEventListener('pointermove', this.onPointerMove);
            this.renderer.domElement.addEventListener('pointerdown', this.onPointerDown);
            this.renderer.domElement.addEventListener('pointercancel', this.onPointerUp);
            this.renderer.domElement.addEventListener('wheel', this.onMouseWheel);
            this.renderer.domElement.addEventListener('click', this.onMouseClick);
            this.renderer.domElement.addEventListener('dblclick', this.onMouseDbClick);
            this.markDirty()
        }
    }

    // 处理鼠标缩放
    onMouseWheel = (event: any) => {
        if (event.deltaY < 0) {
            this.camera2d.zoom = Math.max(minZoom, Math.min(maxZoom, this.camera2d.zoom * dollyScale));
            this.camera2d.updateProjectionMatrix();
        } else if (event.deltaY > 0) {
            this.camera2d.zoom = Math.max(minZoom, Math.min(maxZoom, this.camera2d.zoom / dollyScale));
            this.camera2d.updateProjectionMatrix();
        }
        zoom = this.camera2d.zoom * scale

    }
    onPointerDown = (event: any) => {
        console.log("onPointerDown ", event)
        this.mousePressed = true
    }
    onPointerUp = (event: any) => {
        console.log("onPointerUp ", event)
        this.mousePressed = false
    }
    onMouseClick = (event: any) => {
        console.log("onMouseClick ", event)
    }
    onMouseDbClick = (event: any) => {
        console.log("onMouseDbClick ", event)
    }
    onPointerMove = (event: any) => {
        console.log("onPointerMove ", event)
        if (this.mousePressed) { // 处理鼠标左键按下 拖动
            this.target_x -= event.movementX / zoom
            this.target_y += event.movementY / zoom

            this.camera2d.position.set(this.target_x, this.target_y, CAMERA_Z);
            this.camera2d.lookAt(new THREE.Vector3(this.target_x, this.target_y, 0));
            this.camera2d.updateProjectionMatrix();
        } else {
            // event.clientX, event.clientY
            let [x, y] = this.translateMousePointToMap(event.clientX, event.clientY)
            console.log(x, y)
        }
    }

    private translateMousePointToMap = (mouse_x: number, mouse_y: number): [number, number] => {
        // 鼠标点 相对于 view中心点的 坐标
        let delta_x = mouse_x - window.innerWidth / 2
        let delta_y = mouse_y - window.innerHeight / 2
        // 转化为地图上相对坐标
        return [this.target_x + delta_x / zoom, this.target_y + delta_y / zoom]
    }

    private translateMapPointToMouse = (map_x: number, map_y: number): [number, number] => {
        let delta_x = map_x - this.target_x
        let delta_y = map_y - this.target_y
        return [window.innerWidth / 2 + delta_x * zoom, window.innerHeight / 2 + delta_y * zoom]
    }
}

let CONTEXT_3D = new ThreedContext();

export {CONTEXT_3D}