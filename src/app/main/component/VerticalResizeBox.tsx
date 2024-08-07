'use client'

import React, { useRef, useState } from 'react';



export function VerticalResizeBox({ children }) {
    // const DEFAULT_WINDOW_WIDTH = window.innerWidth / 12;

    const [width, setWidth] = useState(200);
    const startXRef = useRef(0);

    const onMouseDown = (e: any) => {
        console.log(e)
        e.preventDefault();
        startXRef.current = e.clientX;

        document.addEventListener("mousemove", onMouseMove);
        document.addEventListener("mouseup", onMouseUp);
    };

    const onMouseMove = (e: any) => {
        const MAX_WINDOW_WIDTH = window.innerWidth * 3 / 5;
        const MIN_WINDOW_WIDTH = window.innerWidth / 24;

        const targetWidth = width + e.clientX - startXRef.current;

        if (targetWidth < MIN_WINDOW_WIDTH) {
            setWidth(1);
        } else if (targetWidth < MAX_WINDOW_WIDTH) {
            setWidth(targetWidth);
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
                'width': `${width}px`,
                'height': '100%',
                'overflow': 'clip',
                'backgroundColor': '#242424',
                'display': 'flex',
                'flexDirection': 'row',
                'flexGrow': '0'
            }}
            id="VerticalResizableBox"
        >
            <div className='grow'>
                {children}
            </div>
            <div className='bg-[#2f2f2f] w-0.5 h-full cursor-ew-resize' onMouseDown={onMouseDown}>
            </div>
        </div >
    );
};