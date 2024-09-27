use crate::scene::camera::Camera;
use iced::{Color, Rectangle};

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    camera_proj: glam::Mat4,
    camera_pos: glam::Vec4,
    light_color: glam::Vec4,
}

impl Uniforms {
    pub fn new(
        camera: &Camera, light_color: Color,
    ) -> Self {
        let camera_proj = camera.build_view_projection_matrix();

        Self {
            camera_proj,
            camera_pos: glam::Vec4::new(
                camera.eye().x,
                camera.eye().y,
                camera.eye().z,
                1.0,
            ),

            light_color: glam::Vec4::from(light_color.into_linear()),
        }
    }
}
