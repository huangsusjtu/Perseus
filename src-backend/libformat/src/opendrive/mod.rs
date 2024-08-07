use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::pretty_xml::prettify_xml;

pub mod common;
pub mod header;
pub mod junction;
pub mod lane;
pub mod object;
pub mod railroad;
pub mod road;
pub mod signal;
pub mod station;
pub mod custom_extension;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenDrive {
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<header::Header>,

    #[serde(rename = "road")]
    pub road: Vec<road::Road>, // not empty >=1
    #[serde(rename = "controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<signal::signal::Controller>>,
    #[serde(rename = "junction", skip_serializing_if = "Option::is_none")]
    pub junction: Option<Vec<junction::Junction>>,
    #[serde(rename = "junctionGroup", skip_serializing_if = "Option::is_none")]
    pub junction_group: Option<Vec<junction::JunctionGroup>>,
    #[serde(rename = "station", skip_serializing_if = "Option::is_none")]
    pub station: Option<Vec<station::Station>>,

    // 自定义的部分
    #[serde(rename = "cleanArea", skip_serializing_if = "Option::is_none")]
    pub clean_area: Option<Vec<custom_extension::CleanArea>>,
}

pub fn parse<P: AsRef<Path>>(path: P) -> anyhow::Result<OpenDrive> {
    let xml = fs::read_to_string(path)?;
    let drive: OpenDrive = quick_xml::de::from_str(&xml)?;
    return Ok(drive);
}

pub fn unparse(instance: &OpenDrive) -> anyhow::Result<String> {
    let mut xml = String::new();
    quick_xml::se::to_writer_with_root(&mut xml, "OpenDRIVE", &instance)?;
    xml = prettify_xml(&mut xml);
    return Ok(xml);
}

#[cfg(test)]
mod tests {
    use std::fs;
    
    use std::path::Path;

    use crate::opendrive::road::Road;
    use crate::pretty_xml;

    use super::*;

    #[test]
    fn it_works_opendrive1() {
        let drive = OpenDrive {
            header: Default::default(),
            road: vec![Road {
                id: "".to_string(),
                junction: "".to_string(),
                length: 0.0,
                name: None,
                rule: None,
                link: None,
                r#type: None,
                plan_view: Default::default(),
                elevation_profile: None,
                lateral_profile: None,
                lanes: Default::default(),
                objects: None,
            }],
            controller: None,
            junction: None,
            junction_group: None,
            station: None,
            clean_area: None,
        };
        let mut xml = String::new();

        quick_xml::se::to_writer_with_root(&mut xml, "OpenDRIVE", &drive).unwrap();
        println!("{:#?}", xml);
    }

    #[test]
    fn it_works_opendrive2() {
        let xml = r#"<OpenDRIVE>
                                <header revMajor="1"
                                        revMinor="5"
                                        name=""
                                        version="1.00"
                                        date="Mon Nov 29 12:59:50 2021"
                                        north="0.0000000000000000e+00"
                                        south="0.0000000000000000e+00"
                                        east="0.0000000000000000e+00"
                                        west="0.0000000000000000e+00">
                                    <defaultRegulations>
                                        <roadRegulations type="motorway">
                                            <semantics>
                                                <speed type="maximum" value="120" unit="km/h"/>
                                            </semantics>
                                        </roadRegulations>
                                        <roadRegulations type="rural">
                                            <semantics>
                                                <speed type="maximum" value="50" unit="km/h"/>
                                            </semantics>
                                        </roadRegulations>
                                        <roadRegulations type="town">
                                            <semantics>
                                                <speed type="maximum" value="30" unit="km/h"/>
                                            </semantics>
                                        </roadRegulations>
                                        <roadRegulations type="livingStreet">
                                            <semantics>
                                                <speed type="maximum" value="5" unit="km/h"/>
                                            </semantics>
                                        </roadRegulations>
                                        <signalRegulations type="1000001" subType="-1">
                                            <semantics>
                                                <priority type="turnOnRedAllowed" />
                                            </semantics>
                                        </signalRegulations>
                                    </defaultRegulations>
                                </header>
                            </OpenDRIVE>"#;
        let drive: OpenDrive = quick_xml::de::from_str(&xml).unwrap();
        let new_drive = OpenDrive::default();
        let mut xml = String::new();
        quick_xml::se::to_writer_with_root(&mut xml, "OpenDRIVE", &drive).unwrap();
        xml = pretty_xml::prettify_xml(&xml);
        println!("{:#?}", xml);
        println!("{:#?}", std::env::current_dir().unwrap());
        fs::write(
            std::env::current_dir().unwrap().join("test").join("it_works_opendrive2.xml"),
            xml,
        )
            .expect("create test result");
    }

    #[test]
    fn it_works_opendrive3() {
        let file_path = std::env::current_dir()
            .unwrap()
            .join("asset")
            .join("opendrive")
            .join("road_straight_curve_junction.xodr");
        let xml = fs::read_to_string(file_path).unwrap();
        let r = quick_xml::de::from_str::<OpenDrive>(&xml).unwrap();
    }

    #[test]
    fn it_works_opendrive4() {
        let target_dir = std::env::current_dir().unwrap().join("asset").join("opendrive");
        let path = Path::new(&target_dir);
        let dir = std::fs::read_dir(path).unwrap();
        for entry in dir {
            // Check if the entry is a file
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                let file_path = entry.path();
                // Process the file here
                println!("Processing file: {:?}", file_path);

                // Optionally, read the file contents
                let xml = fs::read_to_string(file_path).unwrap();
                let r = quick_xml::de::from_str::<OpenDrive>(&xml).unwrap();
                // match r {
                //     Ok(drive) => {
                //         println!("{:?}", drive.header.name);
                //     }
                //     Err(e) => {
                //         println!("{:#?}", e);
                //     }#[serde(flatten)]
                // }
            }
        }
    }


    // #[test]
    // fn it_works_opendrive7() {
    //     let case = "xxxx";
    //     let file_path = std::env::current_dir().unwrap().join("asset").join("opendrive").join(case);
    //     let xml = fs::read_to_string(file_path).unwrap();
    // }

    #[test]
    fn it_works_opendrive8() {
        let target_dir = std::env::current_dir().unwrap().join("asset").join("opendrive");
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
            let xml = fs::read_to_string(target_dir.join(file)).unwrap();
            let r = quick_xml::de::from_str::<OpenDrive>(&xml);
            if r.is_err() {
                println!("Processing file: {:?}", file);
                r.unwrap();
            }
        }
    }
}
