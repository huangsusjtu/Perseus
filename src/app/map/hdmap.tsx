
import React, { useEffect, useRef } from "react";
import * as THREE from "three";
import { CONTEXT_3D } from "./control/controller";



function HdMapView() {
    const refContainer = useRef(null);
    const inited = useRef<boolean>(false);

    useEffect(() => {
        if (!inited.current) {
            inited.current = true;
            CONTEXT_3D.init()
            if (refContainer.current) {
                refContainer.current.appendChild(CONTEXT_3D.renderer.domElement);
                console.log("ok")
            } else {
                console.log("err")
            }
        }

    }, []);



    return (
        <div ref={refContainer}>

        </div>
    );
}

export { HdMapView };




