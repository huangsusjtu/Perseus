mod camera;
mod event_dispatch;
mod geometry;
mod pipeline;
mod primitive;
mod texture;
mod uniform;
mod vertex;

use crate::scene::camera::{Camera, CameraController};
use crate::scene::event_dispatch::EventDispatch;
use iced::advanced::graphics::core::{event, keyboard};
use iced::advanced::{mouse, Shell};
use iced::keyboard::Key;
use iced::mouse::{Button, Cursor, ScrollDelta};
use iced::touch::Event;
use iced::widget::shader;
use iced::window::RedrawRequest;
use iced::Rectangle;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Scene3d {
    map: libmap::MapRef,
    camera: Arc<Mutex<Camera>>,
    event_dispatch: Arc<Mutex<EventDispatch>>,
}

impl Scene3d {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let camera = Arc::new(Mutex::new(Camera::new_orthographic(
            glam::Vec3::new(0.0, 0.0, 200.0),
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::new(0.0, 1.0, 0.0),
            0.1,
            1000.0,
            800.0,
        )));
        // camera: Camera::new_perspective(
        //     glam::Vec3::new(0.0, 0.0, 200.0),
        //     glam::Vec3::new(0.0, 0.0, 0.0),
        //     glam::Vec3::new(0.0, 1.0, 0.0),
        //     0.1,
        //     1000.0,
        //     60.0,
        // ),
        let camera_controller = CameraController::new(camera.clone());
        let event_dispatch: Arc<Mutex<EventDispatch>> = Default::default();
        event_dispatch.lock().unwrap().register(camera_controller);
        let open_drive = libformat::opendrive::parse(path).unwrap();
        Scene3d {
            map: Arc::new(libmap::SDMap::from(open_drive)),
            camera: camera.clone(),
            event_dispatch,
        }
    }
}

impl<Message> shader::Program<Message> for Scene3d {
    type State = ();
    type Primitive = primitive::Primitive;
    fn draw(
        &self, state: &Self::State, cursor: Cursor, bounds: Rectangle,
    ) -> Self::Primitive {
        self.camera.lock().unwrap().set_viewport(bounds);

        primitive::Primitive::new(
            self.map.clone(),
            self.camera.lock().unwrap().clone(),
        )
    }

    /// 事件分发
    fn update(
        &self, _state: &mut Self::State, event: shader::Event,
        bounds: Rectangle, cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
    ) -> (event::Status, Option<Message>) {
        let mut status = event::Status::Ignored;

        // view port for camera
        self.camera.lock().unwrap().set_viewport(bounds);
        match event {
            shader::Event::Mouse(m_event) => {
                match m_event {
                    iced::mouse::Event::CursorEntered => {}
                    iced::mouse::Event::CursorLeft => {}
                    iced::mouse::Event::CursorMoved { position } => {
                        self.event_dispatch
                            .lock()
                            .unwrap()
                            .on_mouse_move(position.x, position.y);
                        status = event::Status::Captured;
                    }
                    iced::mouse::Event::ButtonPressed(button) => match button {
                        Button::Left => {
                            self.event_dispatch
                                .lock()
                                .unwrap()
                                .on_mouse_left_pressed();
                            status = event::Status::Captured;
                        }
                        Button::Right => {
                            self.event_dispatch
                                .lock()
                                .unwrap()
                                .on_mouse_right_pressed();
                            status = event::Status::Captured;
                        }
                        Button::Middle => {}
                        Button::Back => {}
                        Button::Forward => {}
                        Button::Other(_) => {}
                    },
                    iced::mouse::Event::ButtonReleased(button) => {
                        match button {
                            Button::Left => {
                                self.event_dispatch
                                    .lock()
                                    .unwrap()
                                    .on_mouse_left_release();
                                status = event::Status::Captured;
                            }
                            Button::Right => {
                                self.event_dispatch
                                    .lock()
                                    .unwrap()
                                    .on_mouse_right_release();
                                status = event::Status::Captured;
                            }
                            Button::Middle => {}
                            Button::Back => {}
                            Button::Forward => {}
                            Button::Other(_) => {}
                        }
                    }
                    iced::mouse::Event::WheelScrolled { delta } => {
                        // tracing::info!("WheelScrolled event {:?}", delta);
                        match delta {
                            ScrollDelta::Lines { x, y } => {
                                self.event_dispatch
                                    .lock()
                                    .unwrap()
                                    .on_wheel_scroll(y);
                                status = event::Status::Captured;
                            }
                            ScrollDelta::Pixels { .. } => {}
                        }
                    }
                }
                // tracing::info!("Mouse event {:?}", m_event);
            }
            shader::Event::Touch(m_event) => {
                tracing::trace!("Touch event {:?}", m_event);
            }
            shader::Event::Keyboard(m_event) => match m_event {
                keyboard::Event::KeyPressed {
                    key,
                    location,
                    modifiers,
                    text,
                } => match key {
                    Key::Named(c) => {}
                    Key::Character(c) => {
                        self.event_dispatch
                            .lock()
                            .unwrap()
                            .on_keyboard_press(c.as_ref());
                        status = event::Status::Captured;
                    }
                    Key::Unidentified => {}
                },
                keyboard::Event::KeyReleased { .. } => {}
                keyboard::Event::ModifiersChanged(_) => {}
            },
            shader::Event::RedrawRequested(m_event) => {
                tracing::trace!("RedrawRequested event {:?}", m_event);
            }
        }
        if status == event::Status::Captured {
            shell.request_redraw(RedrawRequest::NextFrame);
        }
        (status, None)
    }
}
