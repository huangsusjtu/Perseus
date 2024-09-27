use crate::scene::Scene3d;
use iced::widget::{column, container, shader};
use iced::Length::Fill;
use iced::{Element, Sandbox};

pub struct PerseusApp {
    pub(crate) scene: Scene3d,
}

impl Sandbox for PerseusApp {
    type Message = Message;

    fn new() -> Self {
        PerseusApp {
            scene: Scene3d::new("/home/huangsu/work/own/Perseus/src-backend/libformat/asset/opendrive/circular_road.xodr") ,
        }
    }

    fn title(&self) -> String {
        "PerseusApp".to_string()
    }

    /// 处理消息
    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let shader = shader(&self.scene).width(Fill).height(Fill);

        container(column![shader]).into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {}
