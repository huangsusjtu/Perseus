use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use xml::attribute::OwnedAttribute;

use crate::openscenario::preprocess::context_tree::ContextNode;
use crate::openscenario::preprocess::validator::read_xsd;

pub mod context_tree;
pub mod expression;
pub mod validator;

pub fn validate_scenario(root: Rc<RefCell<ContextNode>>) -> anyhow::Result<()> {
    let rules = read_xsd(include_str!("OpenSCENARIO.xsd"))?;
    let errs = validator::validate_scenario("OpenScenario", root, &rules)?;
    if !errs.is_empty() {
        return Err(anyhow::anyhow!(format!("{:#?}", errs)));
    }
    return Ok(());
}

fn attrs_to_map(attributes: &Vec<OwnedAttribute>) -> HashMap<String, String> {
    attributes.iter().map(|v| (v.name.local_name.clone(), v.value.clone())).collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::openscenario::parse;

    #[test]
    fn it_works1() {
        let r = parse(std::env::current_dir()
            .unwrap()
            .join("asset")
            .join("openscenario")
            .join("speed-profile.xosc"));
        if r.is_err() {
            r.as_ref().unwrap();
        }
        println!("{:#?}", r);
    }

    #[test]
    fn it_works2() {
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
            // println!("Processing file: {:?}", file);
            let r = parse(target_dir.join(file));
            if r.is_err() {
                println!("{:#?}", file);
                r.as_ref().unwrap();
            }
            // println!("{:#?}", r);
        }
    }
}
