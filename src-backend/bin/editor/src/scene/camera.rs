use crate::scene::event_dispatch::EventListener;
use iced::Rectangle;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Camera {
    eye: glam::Vec3,
    target: glam::Vec3,
    up: glam::Vec3,
    z_near: f32,
    z_far: f32,

    r#type: CameraType,

    viewport: Option<Rectangle>,
}

#[derive(Clone, Debug)]
enum CameraType {
    Perspective { fovy: f32 },
    Orthographic { height: f32 },
}

impl Camera {
    pub fn new_perspective(
        eye: glam::Vec3, target: glam::Vec3, up: glam::Vec3, z_near: f32,
        z_far: f32, fovy: f32,
    ) -> Self {
        Camera {
            eye,
            target,
            up,
            z_near,
            z_far,
            r#type: CameraType::Perspective { fovy },
            viewport: None,
        }
    }
    pub fn new_orthographic(
        eye: glam::Vec3, target: glam::Vec3, up: glam::Vec3, z_near: f32,
        z_far: f32, height: f32,
    ) -> Self {
        Camera {
            eye,
            target,
            up,
            z_near,
            z_far,
            r#type: CameraType::Orthographic { height },
            viewport: None,
        }
    }
    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        let aspect_ratio =
            self.viewport.unwrap().width / self.viewport.unwrap().height;
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = match self.r#type {
            CameraType::Perspective { fovy } => glam::Mat4::perspective_rh(
                fovy.to_radians(),
                aspect_ratio,
                self.z_near,
                self.z_far,
            ),
            CameraType::Orthographic { height } => glam::Mat4::orthographic_rh(
                -0.5 * aspect_ratio * height,
                0.5 * aspect_ratio * height,
                -0.5 * height,
                0.5 * height,
                self.z_near,
                self.z_far,
            ),
        };

        proj * view
    }

    pub fn build_projection_matrix(&self) -> glam::Mat4 {
        let aspect_ratio =
            self.viewport.unwrap().width / self.viewport.unwrap().height;
        let proj = match self.r#type {
            CameraType::Perspective { fovy } => glam::Mat4::perspective_rh(
                fovy.to_radians(),
                aspect_ratio,
                self.z_near,
                self.z_far,
            ),
            CameraType::Orthographic { height } => glam::Mat4::orthographic_rh(
                -0.5 * aspect_ratio * height,
                0.5 * aspect_ratio * height,
                -0.5 * height,
                0.5 * height,
                self.z_near,
                self.z_far,
            ),
        };

        proj
    }
    pub fn build_view_matrix(&self) -> glam::Mat4 {
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        view
    }

    pub fn eye(&self) -> glam::Vec3 {
        self.eye
    }

    pub fn target(&self) -> glam::Vec3 {
        self.target
    }

    pub fn up(&self) -> glam::Vec3 {
        self.up
    }

    pub fn set_eye(&mut self, eye: glam::Vec3) {
        self.eye = eye;
    }

    pub fn set_target(&mut self, target: glam::Vec3) {
        self.target = target;
    }

    pub fn set_up(&mut self, up: glam::Vec3) {
        self.up = up;
    }

    pub fn set_viewport(&mut self, viewport: Rectangle) {
        self.viewport = Some(viewport);
    }
}

pub struct CameraController {
    camera: Arc<Mutex<Camera>>,
    speed: f32,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(camera: Arc<Mutex<Camera>>) -> Self {
        Self {
            camera,
            speed: 0.1,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }
}

impl EventListener for CameraController {
    fn on_wheel_scroll(&mut self, zoom_delta: f32) {
        let mut camera = self.camera.lock().unwrap();
        let forward = camera.target() - camera.eye();
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        match camera.r#type {
            CameraType::Perspective { .. } => {
                camera.eye += forward_norm * (zoom_delta * forward_mag * 0.1); // todo
            }
            CameraType::Orthographic { height } => {
                camera.r#type = CameraType::Orthographic {
                    height: height - zoom_delta * height * self.speed,
                };
            }
        }
    }

    fn on_mouse_move(&mut self, x: f32, y: f32, delta_x: f32, delta_y: f32) {
        if self.is_right_pressed {
            let mut camera = self.camera.lock().unwrap();
            let height = match camera.r#type {
                CameraType::Perspective { .. } => return,
                CameraType::Orthographic { height } => height,
            };

            let physic_per_pixel = height / camera.viewport.unwrap().height;

            let up = camera.up;
            let forward = camera.target() - camera.eye();
            let forward_norm = forward.normalize();
            let right = forward_norm.cross(up);
            let m = -right * delta_x * physic_per_pixel
                + up * delta_y * physic_per_pixel;
            camera.eye += m;
            camera.target += m;
        }
    }
    fn on_keyboard_press(&self, c: &str) {
        let mut camera = self.camera.lock().unwrap();
        let forward = camera.target() - camera.eye();
        let forward_norm = forward.normalize();
        let forward_mag = match camera.r#type {
            CameraType::Perspective { .. } => forward.length(),
            CameraType::Orthographic { height } => height,
        };

        let up = camera.up.clone();
        if c.eq("w") {
            camera.eye += up * forward_mag * self.speed;
            camera.target += up * forward_mag * self.speed;
        } else if c.eq("s") {
            camera.eye -= up * forward_mag * self.speed;
            camera.target -= up * forward_mag * self.speed;
        } else if c.eq("a") {
            let forward = camera.target() - camera.eye();
            let forward_norm = forward.normalize();
            let right = forward_norm.cross(camera.up);
            camera.eye -= right * forward_mag * self.speed;
            camera.target -= right * forward_mag * self.speed;
        } else if c.eq("d") {
            let forward = camera.target() - camera.eye();
            let forward_norm = forward.normalize();
            let right = forward_norm.cross(camera.up);
            camera.eye += right * forward_mag * self.speed;
            camera.target += right * forward_mag * self.speed;
        }
    }
    fn on_mouse_left_pressed(&mut self) {
        self.is_left_pressed = true;
    }
    fn on_mouse_left_release(&mut self) {
        self.is_left_pressed = false;
    }
    fn on_mouse_right_pressed(&mut self) {
        self.is_right_pressed = true;
    }
    fn on_mouse_right_release(&mut self) {
        self.is_right_pressed = false;
    }
}
