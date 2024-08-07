use std::fmt::Formatter;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use dashmap::mapref::entry::Entry;
use dashmap::DashMap;

use crate::eventbus::key::EventKey;
use crate::eventbus::value::EventValue;

pub trait Handler {
    fn on_event(&self, _: &EventValue);
}

impl<F> Handler for F
where
    F: Fn(&EventValue),
{
    fn on_event(&self, arg: &EventValue) {
        (*self)(arg);
    }
}

#[derive(Clone)]
struct EventHandler {
    id: usize,
    handler: Arc<dyn Handler>,
}

impl std::fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventHandler id:{} ", self.id)
    }
}

// EventBus
#[derive(Debug)]
pub struct EventBusImpl {
    id: AtomicUsize,
    handlers: DashMap<EventKey, Vec<EventHandler>>,
}

unsafe impl Send for EventBusImpl {}

unsafe impl Sync for EventBusImpl {}

impl Default for EventBusImpl {
    fn default() -> Self {
        EventBusImpl::new()
    }
}

impl EventBusImpl {
    pub fn new() -> Self {
        EventBusImpl {
            id: AtomicUsize::new(0),
            handlers: DashMap::new(),
        }
    }

    // 注册事件接收对象
    pub fn register(
        &self, type_: EventKey, handler_: Arc<dyn Handler>,
    ) -> usize {
        let id = self.id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let event_handler = EventHandler {
            id: id,
            handler: handler_,
        };
        match self.handlers.entry(type_) {
            Entry::Occupied(inner) => {
                inner.into_ref().push(event_handler);
            }
            Entry::Vacant(inner) => {
                let mut handler_vec = Vec::with_capacity(10);
                handler_vec.push(event_handler);
                inner.insert(handler_vec);
            }
        };
        tracing::trace!("register success, event type is :{:?}", type_);
        return id;
    }

    // 取消注册事件接收对象
    pub fn unregister(&self, type_: EventKey, id_: usize) {
        let event_vec = self.handlers.get_mut(&type_);
        if event_vec.is_none() {
            return;
        }
        let mut event_vec = event_vec.unwrap();
        for i in 0..event_vec.len() {
            if event_vec[i].id == id_ {
                event_vec.remove(i);
                return;
            }
        }
    }

    // 取消某种事件类型的所有监听
    pub fn unregister_one_type(&self, type_: EventKey) {
        let event_vec = self.handlers.get_mut(&type_);
        if event_vec.is_none() {
            return;
        }
        let mut event_vec = event_vec.unwrap();
        event_vec.clear();
    }

    // 取消所有事件监听
    pub fn unregister_all(&self) {
        if self.handlers.is_empty() {
            return;
        }
        self.handlers.clear();
    }

    // 发送特定类型的事件
    pub fn post_event(&self, type_: EventValue) {
        let event_vec = {
            self.handlers
                .get_mut(&(&type_).into())
                .map(|v| v.value().iter().cloned().collect::<Vec<_>>())
        };
        let Some(event_vec) = event_vec else {
            return;
        };
        for handler in event_vec {
            handler.handler.on_event(&type_);
        }
    }
}

#[cfg(test)]
mod tests {

    // 主线程发送
    // #[test]
    // fn test_traffic_light() {
    //     fn add(arg: &EventValue) {
    //         println!("add traffic light: {:?}", arg);
    //     }
    //     let bus = EventBus::new();
    //     let type_ = EventKey::TrafficLight(TrafficLightEvent::UpdateStatus);
    //     let data_ = TrafficLight {
    //         id_: String::from("100"),
    //         trigger_position_: None,
    //         items_: Vec::new(),
    //         have_triggered: false,
    //     };
    //
    //     let id = bus.register(type_, Arc::new(add));
    //     bus.post_event(EventValue::TrafficLight(TrafficLightValue::UpdateStatus(data_)));
    //     bus.unregister(type_, id);
    //     bus.post_event(EventValue::TrafficLight(TrafficLightValue::UpdateStatus(
    //         TrafficLight {
    //             id_: String::from("200"),
    //             trigger_position_: None,
    //             items_: Vec::new(),
    //             have_triggered: false,
    //         },
    //     )));
    //
    //     let _id1 = bus.register(
    //         type_,
    //         Arc::new(|arg: &EventValue| {
    //             println!("1 received, arg:{:?}", arg);
    //         }),
    //     );
    //     let _id2 = bus.register(
    //         type_,
    //         Arc::new(|arg: &EventValue| {
    //             println!("2 received, arg:{:?}", arg);
    //         }),
    //     );
    //
    //     bus.post_event(EventValue::TrafficLight(TrafficLightValue::UpdateStatus(
    //         TrafficLight {
    //             id_: String::from("300"),
    //             trigger_position_: None,
    //             items_: Vec::new(),
    //             have_triggered: false,
    //         },
    //     )));
    //
    //     bus.unregister_one_type(type_);
    //     bus.post_event(EventValue::TrafficLight(TrafficLightValue::UpdateStatus(
    //         TrafficLight {
    //             id_: String::from("400"),
    //             trigger_position_: None,
    //             items_: Vec::new(),
    //             have_triggered: false,
    //         },
    //     )));
    // }
}
