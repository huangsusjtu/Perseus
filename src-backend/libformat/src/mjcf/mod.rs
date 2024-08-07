use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub mod actuator;
pub mod asset;
pub mod body;
pub mod contact;
pub mod custom;
pub mod default;
pub mod deformable;
pub mod equality;
pub mod extension;
pub mod keyframe;
pub mod meta;

pub mod sensor;
pub mod tendon;
pub mod util;
pub mod visual;

/// This is the root MJCF Format element.
#[derive(Deserialize, Serialize, Debug)]
pub struct Mujoco {
    #[serde(rename = "@model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(rename = "include", skip_serializing_if = "Option::is_none")]
    pub include: Option<meta::Include>,

    #[serde(rename = "option", skip_serializing_if = "Option::is_none")]
    pub option: Option<Vec<meta::OptionType>>,
    #[serde(rename = "compiler", skip_serializing_if = "Option::is_none")]
    pub compiler: Option<Vec<meta::Compiler>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<meta::Size>>,
    #[serde(rename = "statistic", skip_serializing_if = "Option::is_none")]
    pub statistic: Option<Vec<meta::Statistic>>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<Vec<asset::Asset>>,
    #[serde(rename = "worldbody", skip_serializing_if = "Option::is_none")]
    pub world_body: Option<body::Body>,
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<Vec<contact::Contact>>,
    #[serde(rename = "deformable", skip_serializing_if = "Option::is_none")]
    pub deformable: Option<Vec<deformable::Deformable>>,
    #[serde(rename = "equality", skip_serializing_if = "Option::is_none")]
    pub equality: Option<Vec<equality::Equality>>,
    #[serde(rename = "tendon", skip_serializing_if = "Option::is_none")]
    pub tendon: Option<Vec<tendon::Tendon>>,
    #[serde(rename = "actuator", skip_serializing_if = "Option::is_none")]
    pub actuator: Option<Vec<actuator::Actuator>>,
    #[serde(rename = "sensor", skip_serializing_if = "Option::is_none")]
    pub sensor: Option<Vec<sensor::Sensor>>,
    #[serde(rename = "sensor", skip_serializing_if = "Option::is_none")]
    pub keyframe: Option<Vec<keyframe::KeyFrame>>,
    #[serde(rename = "visual", skip_serializing_if = "Option::is_none")]
    pub visual: Option<Vec<visual::Visual>>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<default::DefaultWrapper>,
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<custom::Custom>>,
    #[serde(rename = "extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<Vec<extension::Extension>>,
}

pub fn parse<P: AsRef<Path>>(path: P) -> anyhow::Result<Mujoco> {
    let path = path.as_ref().to_path_buf();
    let xml = fs::read_to_string(&path)?;
    let mujoco: Mujoco = quick_xml::de::from_str(&xml)?;

    let base_path = path.parent().unwrap();
    merge_include_recursive(mujoco, base_path)
}

pub fn unparse(instance: &Mujoco) -> anyhow::Result<String> {
    let mut xml = String::new();
    quick_xml::se::to_writer_with_root(&mut xml, "mujoco", &instance)?;
    return Ok(xml);
}

fn merge_include_recursive(
    mut root: Mujoco, base_path: &Path,
) -> anyhow::Result<Mujoco> {
    if let Some(include) = root.include.as_ref() {
        let sub_path: PathBuf = base_path.join(&include.file);
        let xml = fs::read_to_string(sub_path)?;
        let mut sub_mujoco =
            merge_include_recursive(quick_xml::de::from_str(&xml)?, base_path)?;

        if sub_mujoco.option.is_some() {
            if root.option.is_some() {
                root.option
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.option.as_mut().unwrap());
            } else {
                root.option = sub_mujoco.option;
            }
        }

        if sub_mujoco.compiler.is_some() {
            if root.compiler.is_some() {
                root.compiler
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.compiler.as_mut().unwrap());
            } else {
                root.compiler = sub_mujoco.compiler;
            }
        }
        if sub_mujoco.size.is_some() {
            if root.size.is_some() {
                root.size
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.size.as_mut().unwrap());
            } else {
                root.size = sub_mujoco.size;
            }
        }
        if sub_mujoco.statistic.is_some() {
            if root.statistic.is_some() {
                root.statistic
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.statistic.as_mut().unwrap());
            } else {
                root.statistic = sub_mujoco.statistic;
            }
        }
        if sub_mujoco.asset.is_some() {
            if root.asset.is_some() {
                root.asset
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.asset.as_mut().unwrap());
            } else {
                root.asset = sub_mujoco.asset;
            }
        }

        if sub_mujoco.world_body.is_some() {
            if root.world_body.is_some() {
                root.world_body
                    .as_mut()
                    .unwrap()
                    .merge_other(sub_mujoco.world_body.unwrap());
            } else {
                root.world_body = sub_mujoco.world_body;
            }
        }

        if sub_mujoco.contact.is_some() {
            if root.contact.is_some() {
                root.contact
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.contact.as_mut().unwrap());
            } else {
                root.contact = sub_mujoco.contact;
            }
        }
        if sub_mujoco.deformable.is_some() {
            if root.deformable.is_some() {
                root.deformable
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.deformable.as_mut().unwrap());
            } else {
                root.deformable = sub_mujoco.deformable;
            }
        }
        if sub_mujoco.equality.is_some() {
            if root.equality.is_some() {
                root.equality
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.equality.as_mut().unwrap());
            } else {
                root.equality = sub_mujoco.equality;
            }
        }

        if sub_mujoco.tendon.is_some() {
            if root.tendon.is_some() {
                root.tendon
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.tendon.as_mut().unwrap());
            } else {
                root.tendon = sub_mujoco.tendon;
            }
        }
        if sub_mujoco.actuator.is_some() {
            if root.actuator.is_some() {
                root.actuator
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.actuator.as_mut().unwrap());
            } else {
                root.actuator = sub_mujoco.actuator;
            }
        }
        if sub_mujoco.sensor.is_some() {
            if root.sensor.is_some() {
                root.sensor
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.sensor.as_mut().unwrap());
            } else {
                root.sensor = sub_mujoco.sensor;
            }
        }
        if sub_mujoco.keyframe.is_some() {
            if root.keyframe.is_some() {
                root.keyframe
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.keyframe.as_mut().unwrap());
            } else {
                root.keyframe = sub_mujoco.keyframe;
            }
        }
        if sub_mujoco.visual.is_some() {
            if root.visual.is_some() {
                root.visual
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.visual.as_mut().unwrap());
            } else {
                root.visual = sub_mujoco.visual;
            }
        }

        if sub_mujoco.default.is_some() {
            if root.default.is_some() {
                root.default
                    .as_mut()
                    .unwrap()
                    .child
                    .append(&mut sub_mujoco.default.as_mut().unwrap().child);
            } else {
                root.default = sub_mujoco.default;
            }
        }

        if sub_mujoco.custom.is_some() {
            if root.custom.is_some() {
                root.custom
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.custom.as_mut().unwrap());
            } else {
                root.custom = sub_mujoco.custom;
            }
        }
        if sub_mujoco.extension.is_some() {
            if root.extension.is_some() {
                root.extension
                    .as_mut()
                    .unwrap()
                    .append(sub_mujoco.extension.as_mut().unwrap());
            } else {
                root.extension = sub_mujoco.extension;
            }
        }
    }

    return Ok(root);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::mjcf::parse;

    #[test]
    fn it_works1() {
        // let target_dir =
        // std::env::current_dir().unwrap().join("asset").join("mjcf");
        // let path = Path::new(&target_dir);

        let path = PathBuf::from_str(
            "/home/huangsu/work/github/robot/mujoco_menagerie/",
        )
            .unwrap();
        let dir = std::fs::read_dir(path).unwrap();
        for entry in dir {
            // Check if the entry is a file
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                let dir_path = entry.path();
                let scene_xml_file_path = dir_path.join("scene.xml");
                if !scene_xml_file_path.exists() {
                    continue;
                }
                // Process the file here
                // println!("Processing file: {:?}", scene_xml_file_path);

                // Optionally, read the file contents
                let r = parse(&scene_xml_file_path);
                match r {
                    Ok(mj) => {
                        // println!("{:?}", &mj.model);
                    }
                    Err(e) => {
                        println!("Processing file: {:?}", scene_xml_file_path);
                        println!("{:#?}", e);
                    }
                }
            }
        }
    }

    #[test]
    fn it_works2() {
        let file_path = PathBuf::from_str(
            "/home/huangsu/work/github/robot/mujoco_menagerie/aloha/scene.xml",
        )
            .unwrap();
        let r = parse(file_path).unwrap();
        // let file_path = PathBuf::from_str(
        //     "/home/huangsu/work/github/robot/mujoco_menagerie/
        // universal_robots_ur5e/scene.xml", )
        //     .unwrap();
        // let r = parse(file_path).unwrap();
        // let file_path = PathBuf::from_str(
        //     "/home/huangsu/work/github/robot/mujoco_menagerie/robotiq_2f85/
        // scene.xml", )
        //     .unwrap();
        // let r = parse(file_path).unwrap();
        // let file_path = PathBuf::from_str(
        //     "/home/huangsu/work/github/robot/mujoco_menagerie/
        // franka_emika_panda/scene.xml", )
        //     .unwrap();
        // let r = parse(file_path).unwrap();
        // let file_path = PathBuf::from_str(
        //     "/home/huangsu/work/github/robot/mujoco_menagerie/ufactory_lite6/
        // scene.xml", )
        //     .unwrap();
        // let r = parse(file_path).unwrap();
        // kuka_iiwa_14
        // skydio_x2
        // unitree_go2
        // rethink_robotics_sawyer
        // flybody
        // ufactory_xarm7
        // hello_robot_stretch
    }
}
