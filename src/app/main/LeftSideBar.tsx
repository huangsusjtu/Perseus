'use client'

import React, { useRef, useState } from 'react';
import Image from 'next/image';

import { VerticalResizeBox } from './component/VerticalResizeBox';

const left_bar_icons =
    [
        {
            key: '1', src: 'asset/change_circle_FILL0_wght700_GRAD0_opsz24.svg', hover_src: 'asset/mood_FILL0_wght700_GRAD0_opsz24.png', last: false,
        },
        {
            key: '2', src: 'asset/home_FILL0_wght700_GRAD0_opsz24.svg', hover_src: 'asset/mood_FILL0_wght700_GRAD0_opsz24.png', last: false,
        },
        {
            key: '3', src: 'asset/rocket_launch_FILL0_wght700_GRAD0_opsz24.svg', hover_src: 'asset/mood_FILL0_wght700_GRAD0_opsz24.png', last: false,
        },
        {
            key: '4', src: 'asset/search_FILL0_wght700_GRAD0_opsz24.svg', hover_src: 'asset/mood_FILL0_wght700_GRAD0_opsz24.png', last: false,
        },
        {
            key: '5', src: 'asset/settings_FILL0_wght700_GRAD0_opsz24.svg', hover_src: 'asset/mood_FILL0_wght700_GRAD0_opsz24.png', last: true,
        },
    ];


export default function LeftSideBar() {
    return (
        <div className='min-h-full flex flex-row ' id='LeftSideBar'>
            <div className='min-h-full bg-[#3b3b3c] flex flex-col w-12 grow-0 justify-items-center'>

                {left_bar_icons.map((item) => (
                    <LeftBarItem key={item.key} data={item} ></LeftBarItem>
                ))}

            </div >

            <VerticalResizeBox>
                <div>123</div>
            </VerticalResizeBox>
        </div >
    )
}

const LeftBarItem = (props: any) => {
    // console.log(props)
    const [hoverd, setHoverd] = useState(false);


    const onMouseOver = (e: any) => {
        setHoverd(true)
    };
    const onMouseOut = (e: any) => {
        setHoverd(false)
    };

    const normal_item_style = 'mx-auto my-4 hover:bg-sky-700';
    const bottom_item_style = 'absolute  bottom-10 left-2 right-2 hover:bg-sky-700';
    return <Image
        src={hoverd ? props.data.hover_src : props.data.src}
        width={32}
        height={32}
        className={props.data.last ? bottom_item_style : normal_item_style}
        alt={props.data.src}
        key={props.data.src}
        onMouseOver={onMouseOver}
        onMouseOut={onMouseOut}
    />;
}

