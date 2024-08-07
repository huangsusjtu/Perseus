use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub mod catalog;

pub mod condition;
pub mod entity;
pub mod header;
pub mod parameter_value_distribution;

mod base_enum;
pub mod common;

mod preprocess;
mod road_network;
pub mod storyboard;

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenScenario {
    // children
    #[serde(rename = "FileHeader", skip_serializing_if = "Option::is_none")]
    pub file_header: Option<header::FileHeader>,

    // ScenarioDefinition
    #[serde(
        rename = "VariableDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_declarations: Option<common::VariableDeclarations>,
    #[serde(
        rename = "MonitorDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub monitor_declarations: Option<common::MonitorDeclarations>,
    #[serde(rename = "CatalogLocations", skip_serializing_if = "Option::is_none")]
    pub catalog_locations: Option<catalog::CatalogLocations>,
    #[serde(rename = "RoadNetwork", skip_serializing_if = "Option::is_none")]
    pub road_network: Option<road_network::RoadNetwork>,
    #[serde(rename = "Entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<entity::Entities>,
    #[serde(rename = "Storyboard", skip_serializing_if = "Option::is_none")]
    pub storyboard: Option<storyboard::Storyboard>,

    // CatalogDefinition
    #[serde(rename = "Catalog", skip_serializing_if = "Option::is_none")]
    pub catalog: Option<catalog::Catalog>,
    // ParameterValueDistributionDefinition
    #[serde(
        rename = "ParameterValueDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_value_distribution:
    Option<parameter_value_distribution::ParameterValueDistribution>,
}

pub fn parse<P: AsRef<Path>>(path: P) -> anyhow::Result<OpenScenario> {
    let xml = fs::read_to_string(path)?;
    let root_context = preprocess::context_tree::xml_to_context_tree(&xml)?;
    preprocess::validate_scenario(root_context.clone())?;
    let new_xml = preprocess::context_tree::context_tree_to_xml(root_context);
    let scenario: OpenScenario = quick_xml::de::from_str(&new_xml)?;
    return Ok(scenario);
}

pub fn unparse(instance: &OpenScenario) -> anyhow::Result<String> {
    let mut xml = String::new();
    quick_xml::se::to_writer_with_root(&mut xml, "OpenSCENARIO", &instance)?;
    return Ok(xml);
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn it_works_process_new_openscenario() {
        let target_dir = std::env::current_dir().unwrap().join("asset").join("openscenario");
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
            let r = parse(target_dir.join(file));
            if r.is_err() {
                // r.unwrap();
                eprintln!("Processing file: {:?}", file);
                eprintln!("{:#?}", r);
            }
        }
    }

    #[test]
    fn it_works_openscenario_single() {
        let target_dir = std::env::current_dir().unwrap().join("asset").join("openscenario");
        let r = parse(target_dir.join("user_defined_action.xosc"));
        if r.is_err() {
            println!("{:#?}", r);
        }
    }

    #[test]
    fn it_works_process_new_openscenario_single() {
        let target_dir = std::env::current_dir().unwrap().join("asset").join("new");
        // let xml = fs::read_to_string(target_dir.join("user_defined_action.xosc")).unwrap();

        let r = parse(target_dir.join("user_defined_action.xosc"));
        if r.is_err() {
            // r.unwrap();
            println!("{:#?}", r);
        }
    }
}
