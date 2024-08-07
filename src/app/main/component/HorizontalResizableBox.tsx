'use client'

import React, { useEffect, useRef, useState } from 'react';



export function HorizontalResizableBox({ children }) {
    // const DEFAULT_WINDOW_HEIGHT = window.innerHeight / 6;
    const [height, setHeight] = useState(200);
    const startYRef = useRef(0);

    const onMouseDown = (e: any) => {
        e.preventDefault();
        startYRef.current = e.clientY;

        document.addEventListener("mousemove", onMouseMove);
        document.addEventListener("mouseup", onMouseUp);
    };

    const onMouseMove = (e: any) => {
        const MAX_WINDOW_HEIGHT = window.innerHeight * 4 / 5;
        const MIN_WINDOW_HEIGHT = window.innerHeight / 24;

        const targetheight = height - (e.clientY - startYRef.current);
        if (targetheight < MIN_WINDOW_HEIGHT) {
            setHeight(1);
        } else if (targetheight < MAX_WINDOW_HEIGHT) {
            setHeight(targetheight);
        } else {
        }
    };

    const onMouseUp = () => {
        document.removeEventListener("mousemove", onMouseMove);
        document.removeEventListener("mouseup", onMouseUp);
    };

    return (
        <div
            style={{
                'height': `${height}px`,
                'width': "100%",
                'overflow': 'clip',
                'backgroundColor': '#141312',
                'display': 'flex',
                'flexDirection': 'column',
                'flexGrow': '0'
            }}
            id="HorizontalResizableBox"
        >
            <div className='bg-[#2f2f2f] w-full h-0.5 cursor-ns-resize' onMouseDown={onMouseDown}>
            </div>

            <div className='grow'>
                {/* {children} */}
                123
            </div>

        </div >
    );
};