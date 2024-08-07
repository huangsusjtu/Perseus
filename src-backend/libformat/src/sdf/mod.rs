use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

mod actor;
mod collision;
mod geometry;
mod joint;
mod light;
mod link;
mod material;
mod model;
mod physics;
mod scene;
mod sensor;
mod state;
mod util;
mod visual;
mod world;

pub type Color = util::Vector4<f64>;

/// This is the root SDFormat element.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Sdf {
    // attribute start
    #[serde(rename = "@version")]
    version: String,
    // attribute end

    // children element start
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub world: Vec<world::World>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub model: Vec<model::Model>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<actor::Actor>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub light: Vec<light::Light>,
    // children element end
}

pub fn parse<P: AsRef<Path>>(path: P) -> anyhow::Result<Sdf> {
    let xml = fs::read_to_string(path)?;
    let sdf: Sdf = quick_xml::de::from_str(&xml)?;
    return Ok(sdf);
}

pub fn unparse(instance: &Sdf) -> anyhow::Result<String> {
    let mut xml = String::new();
    quick_xml::se::to_writer_with_root(&mut xml, "sdf", &instance)?;
    return Ok(xml);
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    const TEST_STR1: &str = r#"<?xml version='1.0'?>
            <sdf version='1.11'>
              <world name='123'>

              </world>
            </sdf>"#;
    const TEST_STR2: &str = r#"<?xml version='1.0'?>
            <sdf version='1.11'>
              <model name='my_model'>
                ...
              </model>
            </sdf>"#;

    const TEST_STR3: &str = r#"<?xml version='1.0'?>
        <sdf version='1.11'>
          <light name='my_light'>
            ...
          </light>
        </sdf>"#;

    const TEST_STR4: &str = r#"<?xml version='1.0'?>
        <sdf version='1.11'>
          <actor name='my_actor'>
            ...
          </actor>
        </sdf>"#;

    #[test]
    fn it_works1() {
        let sdf1: Sdf = quick_xml::de::from_str(TEST_STR1).unwrap();
        // println!("{:#?}", xml);
        // assert_eq!(
        //     "<Actor
        // name=\"123\"><skin><filename>hehe</filename><scale>1</scale></skin></
        // Actor>",     xml
        // );
    }

    #[test]
    fn it_works2() {
        let sdf1: Sdf = quick_xml::de::from_str(TEST_STR2).unwrap();
    }

    #[test]
    fn it_works3() {
        let sdf1: Sdf = quick_xml::de::from_str(TEST_STR3).unwrap();
        // println!("{:#?}", xml);
        // assert_eq!(
        //     "<Actor
        // name=\"123\"><skin><filename>hehe</filename><scale>1</scale></skin></
        // Actor>",     xml
        // );
    }

    #[test]
    fn it_works4() {
        let sdf1: Sdf = quick_xml::de::from_str(TEST_STR4).unwrap();
        // println!("{:#?}", xml);
        // assert_eq!(
        //     "<Actor
        // name=\"123\"><skin><filename>hehe</filename><scale>1</scale></skin></
        // Actor>",     xml
        // );
    }

    #[test]
    fn it_works_5() {
        let target_dir = std::env::current_dir().unwrap().join("asset").join("sdf");
        let path = Path::new(&target_dir);
        let dir = std::fs::read_dir(path).unwrap();
        let mut file_name_vec = Vec::new();
        for entry in dir {
            // Check if the entry is a file
            let entry = entry.unwrap();
            entry.file_name().to_str().unwrap().to_string();

            if entry.file_type().unwrap().is_file() {
                file_name_vec.push(entry.file_name().to_str().unwrap().to_string());
            }
        }
        file_name_vec.sort();

        for file in file_name_vec.iter() {
            println!("Processing file: {:?}", file);
            // let xml = fs::read_to_string(target_dir.join(file)).unwrap();
            let r = parse(target_dir.join(file));
            if r.is_err() {
                println!("Processing file: {:?}", file);
                r.unwrap();
            }
        }
    }
}
