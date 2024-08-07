'use client'

import PannelContainer from "./PannelContainer";
import LeftSideBar from "./LeftSideBar"
import StatusBar from "./StatusBar";
import TitleBar from "./TitleBar"
import ToolBar from "./ToolBar";

export async function getInitialProps() {
    return {
        // 关闭 SSR
        ssr: false,
    };
}

export default function MainPage() {
    return (
        <div className='h-full flex flex-col '>

            <TitleBar></TitleBar>
            <div className='grow h-full flex flex-row '>
                <LeftSideBar></LeftSideBar>
                <div className='grow h-full flex flex-col '>
                    <PannelContainer></PannelContainer>
                    <ToolBar></ToolBar>
                </div>
            </div>


            <StatusBar></StatusBar>
        </div>
    )
}
