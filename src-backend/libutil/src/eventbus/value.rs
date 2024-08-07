use crate::eventbus::EventKey;

#[derive(Default, Debug, Clone, PartialEq, Copy, Hash)]
pub enum EventValue {
    #[default]
    NONE,
}

impl Into<EventKey> for &EventValue {
    fn into(self) -> EventKey {
        match self {
            EventValue::NONE => EventKey::NONE,
        }
    }
}
