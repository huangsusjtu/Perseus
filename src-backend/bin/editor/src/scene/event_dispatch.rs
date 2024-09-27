use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct EventDispatch {
    current_cursor: Option<[f32; 2]>,
    listener: HashMap<TypeId, Box<dyn EventListener + Send>>,
}

impl Default for EventDispatch {
    fn default() -> Self {
        EventDispatch {
            current_cursor: None,
            listener: Default::default(),
        }
    }
}

impl EventDispatch {
    pub fn register<T: 'static + Send + EventListener>(&mut self, listener: T) {
        let _ = self.listener.insert(TypeId::of::<T>(), Box::new(listener));
    }

    pub fn unregister<T: 'static + Send + EventListener>(
        &mut self, listener: T,
    ) {
        self.listener.remove(&TypeId::of::<T>());
    }

    pub fn on_wheel_scroll(&mut self, zoom_delta: f32) {
        for i in self.listener.iter_mut() {
            i.1.on_wheel_scroll(zoom_delta);
        }
    }

    pub fn on_mouse_left_pressed(&mut self) {
        for i in self.listener.iter_mut() {
            i.1.on_mouse_left_pressed()
        }
    }
    pub fn on_mouse_left_release(&mut self) {
        for i in self.listener.iter_mut() {
            i.1.on_mouse_left_release()
        }
    }

    pub fn on_mouse_right_pressed(&mut self) {
        for i in self.listener.iter_mut() {
            i.1.on_mouse_right_pressed()
        }
    }
    pub fn on_mouse_right_release(&mut self) {
        for i in self.listener.iter_mut() {
            i.1.on_mouse_right_release()
        }
    }

    pub fn on_mouse_move(&mut self, x: f32, y: f32) {
        if let Some(t) = self.current_cursor.as_mut() {
            let old = self.current_cursor.unwrap();
            self.current_cursor = Some([x, y]);
            for i in self.listener.iter_mut() {
                i.1.on_mouse_move(x, y, x - old[0], y - old[1])
            }
        } else {
            self.current_cursor = Some([x, y]);
        }
    }

    pub fn on_keyboard_press(&mut self, c: &str) {
        for i in self.listener.iter_mut() {
            i.1.on_keyboard_press(c);
        }
    }
}

pub trait EventListener {
    fn on_wheel_scroll(&mut self, zoom_delta: f32) {}
    fn on_mouse_left_pressed(&mut self) {}
    fn on_mouse_left_release(&mut self) {}
    fn on_mouse_right_pressed(&mut self) {}
    fn on_mouse_right_release(&mut self) {}
    fn on_mouse_move(&mut self, x: f32, y: f32, delta_x: f32, delta_y: f32) {}
    fn on_keyboard_press(&self, c: &str) {}
}
