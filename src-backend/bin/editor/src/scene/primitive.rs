use crate::scene::camera::Camera;
use crate::scene::geometry::{
    generate_road_center_line, generate_road_lane, merge_geometry,
};
use crate::scene::pipeline::PipelineOpenDrive;
use iced::widget::shader;
use iced::widget::shader::wgpu::{
    CommandEncoder, Device, Queue, TextureFormat, TextureView,
};
use iced::widget::shader::Storage;
use iced::{Color, Rectangle, Size};

#[derive(Debug)]
pub struct Primitive {
    map: libmap::MapRef,
    camera: Camera,
}
unsafe impl Sync for Primitive {}
unsafe impl Send for Primitive {}
impl Primitive {
    pub fn new(map: libmap::MapRef, camera: Camera) -> Primitive {
        Primitive { map, camera }
    }
}

impl shader::Primitive for Primitive {
    fn prepare(
        &self, format: TextureFormat, device: &Device, queue: &Queue,
        bounds: Rectangle, target_size: Size<u32>, scale_factor: f32,
        storage: &mut Storage,
    ) {
        // if !storage.has::<PipelineTriangle>() {
        //     let face = self.generate_road_geometry();
        //     storage.store(PipelineTriangle::new(device, queue, format));
        //
        //     //upload data to GPU
        //     let pipeline = storage.get_mut::<PipelineTriangle>().unwrap();
        //     pipeline.update(device, queue, target_size);
        // }

        // if !storage.has::<PipelineFace>() {
        //     let face = self.generate_road_geometry();
        //     storage.store(PipelineFace::new(device, queue, format, face));
        //
        //     //upload data to GPU
        //     let pipeline = storage.get_mut::<PipelineFace>().unwrap();
        //     pipeline.update(device, queue, target_size);
        // }

        {
            // 是否需要重新渲染
            let mut need_new_pipeline = if !storage.has::<libmap::MapRef>() {
                true
            } else {
                let map_ref = storage.get::<libmap::MapRef>().unwrap();
                !map_ref.header.name.eq(&self.map.header.name)
            };
            need_new_pipeline =
                need_new_pipeline || !storage.has::<PipelineOpenDrive>();

            if need_new_pipeline {
                // let face = self.generate_road_geometry();
                let road_lines =
                    generate_road_center_line(self.map.clone(), 0.1);
                let lanes = generate_road_lane(self.map.clone());
                let lines = merge_geometry(lanes, road_lines);
                storage.store(PipelineOpenDrive::new(
                    device, queue, format, lines,
                ));
            }
            //upload data to GPU
            let pipeline = storage.get_mut::<PipelineOpenDrive>().unwrap();
            pipeline.update(device, queue, target_size, &self.camera);
        }
    }

    fn render(
        &self, storage: &Storage, target: &TextureView, target_size: Size<u32>,
        viewport: Rectangle<u32>, encoder: &mut CommandEncoder,
    ) {
        // {
        //     //at this point our pipeline should always be initialized
        //     let pipeline = storage.get::<PipelineTriangle>().unwrap();
        //     //render primitive
        //     pipeline.render(target, encoder, viewport);
        // }
        // {
        //     //at this point our pipeline should always be initialized
        //     let pipeline = storage.get::<PipelineFace>().unwrap();
        //     //render primitive
        //     pipeline.render(target, encoder, viewport);
        // }

        {
            //at this point our pipeline should always be initialized
            let pipeline = storage.get::<PipelineOpenDrive>().unwrap();
            //render primitive
            pipeline.render(target, encoder, viewport);
        }
    }
}
