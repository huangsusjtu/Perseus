'use client'

import { useEffect, useRef, useState } from "react";
import Image from 'next/image';
import ReactDOM from "react-dom";

import React from "react";

// import init, { render_example, add } from "libexample";
import init, { run_3dgs } from "lib3dgs";


function generateRandomString(length: number): string {
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let result = '';
    const charactersLength = characters.length;
    for (let i = 0; i < length; i++) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
}

const onSplitH = (pannel_id: string) => {
    const holder = document.createElement('div') as HTMLDivElement;

    console.log("onSplitH", pannel_id)
    const pannel = document.getElementById(pannel_id)! as HTMLDivElement;
    if (pannel.getBoundingClientRect().width < window.innerWidth / 8) {
        alert("宽度很小,已经无法分割!")
        return;
    }
    let old_parent = pannel?.parentNode!;
    old_parent.replaceChild(holder, pannel);

    const new_parent = document.createElement('div') as HTMLDivElement;
    new_parent.style.height = pannel.style.height;
    new_parent.style.width = pannel.style.width;
    new_parent.style.display = "flex";
    new_parent.style.flexDirection = "row";
    pannel.style.height = "100%";
    pannel.style.width = "50%";

    const id: string = generateRandomString(10);
    const temp1 = document.createElement('div') as HTMLDivElement;
    ReactDOM.render(<SplitLine id={'splitline-' + id} direction={'h'} ></SplitLine>, temp1);
    const splitline = temp1.childNodes[0] as HTMLDivElement;
    temp1.removeChild(splitline);


    const temp2 = document.createElement('div') as HTMLDivElement;
    ReactDOM.render(<Pannel id={'pannel-' + id} ><canvas id={'canvas-' + id} className='w-full h-full'></canvas></Pannel>, temp2);
    const sibling = temp2.childNodes[0] as HTMLDivElement;
    temp2.removeChild(sibling);
    sibling.style.height = "100%";
    sibling.style.width = "50%";

    new_parent.appendChild(pannel);
    new_parent.appendChild(splitline);
    new_parent.appendChild(sibling);
    old_parent.replaceChild(new_parent, holder);
};

const onSplitV = (pannel_id: string) => {
    const holder = document.createElement('div') as HTMLDivElement;

    console.log("onSplitV", pannel_id)
    const pannel = document.getElementById(pannel_id)! as HTMLDivElement;
    if (pannel.getBoundingClientRect().height < window.innerHeight / 8) {
        alert("高度很小,已经无法分割!")
        return;
    }
    let old_parent = pannel?.parentNode!;
    old_parent.replaceChild(holder, pannel);


    const new_parent = document.createElement('div') as HTMLDivElement;
    new_parent.style.height = pannel.style.height;
    new_parent.style.width = pannel.style.width;
    new_parent.style.display = "flex";
    new_parent.style.flexDirection = "column";
    pannel.style.width = "100%";
    pannel.style.height = "50%";

    const id: string = generateRandomString(10);

    const temp1 = document.createElement('div') as HTMLDivElement;
    ReactDOM.render(<SplitLine id={'splitline-' + id} direction={'v'} ></SplitLine>, temp1);
    const splitline = temp1.childNodes[0] as HTMLDivElement;
    temp1.removeChild(splitline);

    const temp2 = document.createElement('div') as HTMLDivElement;
    ReactDOM.render(<Pannel id={'pannel-' + id} ><canvas id={'canvas-' + id} className='w-full h-full'></canvas></Pannel>, temp2);
    const sibling = temp2.childNodes[0] as HTMLDivElement;
    temp2.removeChild(sibling);
    sibling.style.width = "100%";
    sibling.style.height = "50%";

    new_parent.appendChild(pannel);
    new_parent.appendChild(splitline);
    new_parent.appendChild(sibling);
    old_parent.replaceChild(new_parent, holder);
};


const onClose = (pannel_id: string) => {
    console.log("onClose", pannel_id)
    const pannel = document.getElementById(pannel_id)! as HTMLDivElement;
    const father = pannel?.parentNode! as HTMLDivElement;
    const grad_father = father?.parentNode;

    if (father.id == 'PannelContainer' && father.childElementCount == 1) {
        console.log("there is no more pannel to remove", pannel_id)
        return;
    }
    if (father.childElementCount !== 3) {
        console.log("this is not a valide parent", pannel_id)
        return;
    }


    const left = father.childNodes[0] as HTMLDivElement;
    const splitline = father.childNodes[1] as HTMLDivElement;
    const right = father.childNodes[2] as HTMLDivElement;
    const sibling = left === pannel ? right : left;
    sibling.style.width = father.style.width;
    sibling.style.height = father.style.height;

    father.removeChild(pannel);
    father.removeChild(splitline);
    father.removeChild(sibling);
    grad_father?.replaceChild(sibling, father);
};

const onResize = (split_line_id: string, x: number | null, y: number | null) => {
    // console.log("onResizeH", split_line_id, delta_x, delta_y)
    const splitline = document.getElementById(split_line_id)! as HTMLDivElement;
    const father = splitline?.parentNode! as HTMLDivElement;
    if (father.childElementCount !== 3) {
        console.log("this is not a valide parent", split_line_id)
        return;
    }

    const left = father.childNodes[0] as HTMLDivElement;
    const right = father.childNodes[2] as HTMLDivElement;

    const rect_parent = father.getBoundingClientRect();
    const rect_left = left.getBoundingClientRect();
    const rect_right = right.getBoundingClientRect();

    // console.log("onResize", rect_parent, rect_left, rect_right)
    if (x !== null) {
        const target_width = Math.max(window.innerWidth / 16, Math.min(x, rect_parent.width - window.innerWidth / 16))
        const left_ratio = target_width * 100 / rect_parent.width;
        left.style.width = `${left_ratio}%`;
        const right_ratio = 100.0 - left_ratio;
        right.style.width = `${right_ratio}%`;
        // console.log("ratio  ", left_ratio, right_ratio)
    }

    if (y !== null) {
        const target_height = Math.max(window.innerHeight / 16, Math.min(y, rect_parent.height - window.innerHeight / 16))
        const left_ratio = target_height * 100 / rect_parent.height;
        left.style.height = `${left_ratio}%`;
        const right_ratio = 100.0 - left_ratio;
        right.style.height = `${right_ratio}%`;
        // console.log("ratio  ", left_ratio, right_ratio)
    }
}
export default class PannelContainer extends React.Component {
    componentDidMount(): void {
        init().then(() => {
            run_3dgs('canvas-1')
        })
    }

    render() {
        return (
            <div style={{ 'flexGrow': '1', 'display': 'flex', 'flexDirection': 'column', 'width': '100%', 'height': '100px' }} id='PannelContainer'>
                <Pannel id={'pannel-1'} ><canvas id={'canvas-1'} className='w-full h-full'></canvas></Pannel>
            </div>
        )
    }
}


interface PannelProps {
    id: string
    children: JSX.Element | string
}

class Pannel extends React.Component<PannelProps> {
    closeRef: React.RefObject<HTMLDivElement> = React.createRef();
    splitVRef: React.RefObject<HTMLDivElement> = React.createRef();
    splitHRef: React.RefObject<HTMLDivElement> = React.createRef();
    // has_register_event: boolean = false;

    constructor(props: PannelProps) {
        super(props);
    }

    close = () => {
        onClose(this.props.id)
    }
    splitv = () => {
        onSplitV(this.props.id)
    }

    splith = () => {
        onSplitH(this.props.id)
    }


    componentDidMount() {
        console.log("Pannel componentDidMount", this.props.id)
        this.closeRef.current?.removeEventListener('click', this.close);
        this.splitVRef.current?.removeEventListener('click', this.splitv);
        this.splitHRef.current?.removeEventListener('click', this.splith);

        this.closeRef.current?.addEventListener('click', this.close);
        this.splitVRef.current?.addEventListener('click', this.splitv);
        this.splitHRef.current?.addEventListener('click', this.splith);
    }

    componentDidUnMount() {
        console.log("Pannel componentDidUnMount", this.props.id)
    }

    render() {
        return (
            <div
                style={{ 'flexGrow': '1', height: '100%', width: '100%', display: 'flex', 'flexDirection': 'column', }} id={this.props.id} >
                < div className='bg-[#ebff9b] flex flex-row' >
                    <div className='grow h-full cursor-grab'></div>
                    <Image
                        src='asset/splitscreen_left_FILL0_wght400_GRAD0_opsz24.svg'
                        width={18}
                        height={18}
                        className='mx-2 my-1 hover:bg-neutral-200 hover:scale-110'
                        alt='split_left_right'
                        // onClick={this.splitH}
                        ref={this.splitHRef}
                        id={this.props.id + 'splith'}
                    />
                    <Image
                        src='asset/splitscreen_top_FILL0_wght400_GRAD0_opsz24.svg'
                        width={18}
                        height={18}
                        className='mx-2 my-1 hover:bg-neutral-200 hover:scale-110'
                        alt='split_top_bottom'
                        // onClick={this.splitV}
                        ref={this.splitVRef}
                        id={this.props.id + 'splitv'}
                    />
                    <Image
                        src='asset/close_FILL0_wght400_GRAD0_opsz24.svg'
                        width={18}
                        height={18}
                        className='mx-2 my-1 hover:bg-neutral-200 hover:scale-110'
                        alt='close'
                        // onClick={this.close}
                        ref={this.closeRef}
                        id={this.props.id + 'close'}
                    />
                </div >

                {this.props.children}
            </ div >
        )
    }
}


interface SplitLineProps {
    id: string,
    direction: string,
}

class SplitLine extends React.Component<SplitLineProps> {
    divRef: React.RefObject<HTMLDivElement> = React.createRef();
    parent_rect: DOMRect = new DOMRect()

    constructor(props: SplitLineProps) {
        super(props);
    }


    onMouseDown = (e: MouseEvent) => {
        console.log("onMouseDown", e)
        e.preventDefault();

        const self = document.getElementById(this.props.id)! as HTMLDivElement
        const parent = self.parentNode! as HTMLDivElement
        this.parent_rect = parent.getBoundingClientRect();
        if (this.props.direction == 'h') {
            document.body.style.cursor = 'ew-resize';
        } else if (this.props.direction == 'v') {
            document.body.style.cursor = 'ns-resize';
        }


        document.addEventListener("mousemove", this.onMouseMove);
        document.addEventListener("mouseup", this.onMouseUp);
    };

    onMouseMove = (e: MouseEvent) => {
        console.log("onMouseMove", e)
        if (this.props.direction == 'h') {
            const x = e.x - this.parent_rect.x
            onResize(this.props.id, x, null)
        } else if (this.props.direction == 'v') {
            const y = e.y - this.parent_rect.y
            onResize(this.props.id, null, y)
        }
    };

    onMouseUp = (e: MouseEvent) => {
        console.log("onMouseUp", e)
        document.removeEventListener("mousemove", this.onMouseMove);
        document.removeEventListener("mouseup", this.onMouseUp);
        document.body.style.cursor = 'auto'
    };



    componentDidMount() {
        console.log("SplitLine componentDidMount", this.props.id)
        this.divRef.current?.removeEventListener('mousedown', this.onMouseDown);
        this.divRef.current?.addEventListener('mousedown', this.onMouseDown);


    }

    componentDidUnMount() {
        console.log("SplitLine componentDidUnMount", this.props.id)
        this.divRef.current?.removeEventListener('mousedown', this.onMouseDown);
    }

    render() {
        if (this.props.direction == 'v') {
            return (
                <div ref={this.divRef} className=' bg-[#dcdcdc] min-h-1 h-1 w-full cursor-ns-resize grow-0' id={this.props.id}>
                </ div>
            )
        } else {
            return (
                <div ref={this.divRef} className='bg-[#dcdcdc] min-w-1 w-1 h-full cursor-ew-resize  grow-0' id={this.props.id}>
                </div>
            )
        }


    }
}
