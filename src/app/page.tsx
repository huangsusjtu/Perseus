'use client'

import React from "react"
import dynamic from "../../node_modules/next/dynamic";
import { MyThree } from "./map/mythree";
import { HdMapView } from "./map/hdmap";

// const HdMapView = dynamic(() => import('./map/hdmap'),
//     {
//         ssr: false,
//     })


export async function getInitialProps() {
    return {
        ssr: false,
    }
}

interface MainMapProps {

}

interface MainMapStates {

}



export default class MainMapPage extends React.Component<MainMapProps, MainMapStates> {
    have_inited: boolean = false;
    baidumap: BMapGL.Map = null;

    constructor(props: MainMapPage) {
        super(props)
        this.state = {}
    }

    componentDidMount(): void {
        // this.initBaiduMap()
    }

    initBaiduMap() {
        if (!this.have_inited) {
            this.baidumap = new BMapGL.Map("baidu_map_container", {
                enableRotate: false,
                enableTilt: false,
            });

            var point = new BMapGL.Point(121.4786, 31.266108);
            this.baidumap.centerAndZoom(point, 15);
            this.baidumap.enableScrollWheelZoom(true);
            this.have_inited = true;
        }
    }


    render(): React.ReactNode {
        return (
            <div className="h-screen flex flex-col flex-grow">

                {/* <MyThree /> */}
                <HdMapView />
                {/* <div className="h-full w-full grow shrink" id="baidu_map_container">

                </div> */}

            </div>
        )
    }
}
