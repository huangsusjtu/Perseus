use crate::scene::Scene3d;
use iced::widget::{column, container, shader, vertical_space};
use iced::Length::{Fill, Fixed};
use iced::{Element, Sandbox};
use std::path::PathBuf;
use std::str::FromStr;

pub struct PerseusApp {
    pub(crate) scene: Scene3d,
    
    // 地图
    map_asset_base_path: PathBuf,
    map_file_asset: Vec<String>,
    selected_map_file: Option<String>,

    // ply文件
    ply_file: Option<PathBuf>,
}

impl Sandbox for PerseusApp {
    type Message = Message;

    fn new() -> Self {
        let map_asset_base_path= PathBuf::from_str(
            "C:\\Users\\huangsu\\RustroverProjects\\Perseus\\src-backend\\libformat\\asset\\opendrive",
        ).unwrap();

        PerseusApp {
            scene: {
                let mut s = Scene3d::new();
                s.select_map(map_asset_base_path.join("curve_road_mixed.xodr"));
                s
            },
            map_asset_base_path: map_asset_base_path.clone(),
            map_file_asset: {
                let mut r = vec![];
                let dir = std::fs::read_dir(map_asset_base_path).unwrap();
                for entry in dir {
                    // Check if the entry is a file
                    let entry = entry.unwrap();
                    if entry.file_type().unwrap().is_file() {
                        let file_path = entry.path();
                        if file_path.is_file() {
                            let name = file_path
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();
                            r.push(name);
                        }
                    }
                }
                r
            },
            selected_map_file: None,
            ply_file: None,
        }
    }

    fn title(&self) -> String {
        "PerseusApp".to_string()
    }

    /// 处理消息
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::MapSelected(map_name) => {
                self.scene.select_map(self.map_asset_base_path.join(&map_name));
                self.selected_map_file = Some(map_name);
            }
            Message::PlySelected(ply_file_name) => {
                self.scene.select_ply_file(&ply_file_name);
                self.ply_file = Some(PathBuf::from(ply_file_name));
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let map_pick = {
            let map_pick_list = iced::widget::pick_list(
                self.map_file_asset.clone(),
                self.selected_map_file.clone(),
                Message::MapSelected,
            )
            .placeholder("map select");
            let map_pick = column![
                vertical_space().height(10),
                "map select",
                map_pick_list,
                vertical_space().height(10),
            ]
            .width(Fill)
            .spacing(5);
            map_pick
        };
        
        let shader = shader(&self.scene).width(Fill).height(Fill);

        container(column![shader, map_pick]).into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    MapSelected(String),
    PlySelected(String),
}
