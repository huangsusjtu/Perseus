use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use xml::EventReader;
use xml::reader::XmlEvent;

use crate::openscenario::preprocess::attrs_to_map;
use crate::openscenario::preprocess::context_tree::ContextNode;

#[derive(Debug)]
pub struct Rule {
    pub tag: String,
    pub required_attrs: HashMap<String, String>,
    /* key is : name, value
                                                     * is type */
    pub optional_attrs: HashMap<String, String>,
    pub required_elements: HashMap<String, String>,
    /* key is : name,
                                                        * value
                                                        * is type */
    pub optional_elements: HashMap<String, String>,
    pub unique_elements: HashMap<String, String>,
}

impl Rule {
    pub fn new(tag: String) -> Self {
        Rule {
            tag,
            required_attrs: Default::default(),
            optional_attrs: Default::default(),
            required_elements: Default::default(),
            optional_elements: Default::default(),
            unique_elements: Default::default(),
        }
    }
}

pub fn read_xsd(xsd: &str) -> anyhow::Result<HashMap<String, Rule>> {
    let mut result: HashMap<String, Rule> = HashMap::new();
    let mut group_tag_rules: HashMap<String, Rule> = HashMap::new();
    let parser = EventReader::new(xsd.as_bytes());

    let mut current_rule: Option<Rule> = None;
    let mut _level = 0;
    let (mut all_element, mut sequence_element, mut uniquqe_element) = (false, true, false);
    let (mut is_group_rule, mut group_level) = (false, 0);
    for t in parser {
        let e = t?;
        match e {
            #[allow(unused_variables)]
            XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            } => {
                _level += 1;
                let m = attrs_to_map(&attributes);

                if name.local_name.eq("complexType") {
                    // let class_name =
                    // attributes[0].name.local_name.clone();
                    if attributes.len() > 0 {
                        let class_name = m.get("name").unwrap().clone();

                        // debug
                        // if class_name.starts_with("
                        // OverrideControllerValueAction") {
                        //     println!("");
                        // }

                        if result.contains_key(&class_name) {
                            return Err(anyhow::anyhow!("dumplicated type {}", &name.local_name));
                        }
                        current_rule = Some(Rule::new(class_name));
                    }
                } else if name.local_name.eq("group") {
                    group_level += 1;
                    if m.contains_key("name") {
                        is_group_rule = true;
                        let class_name = m.get("name").unwrap().clone();
                        if result.contains_key(&class_name) {
                            return Err(anyhow::anyhow!("dumplicated type {}", &name.local_name));
                        }
                        current_rule = Some(Rule::new(class_name));
                    } else if m.contains_key("ref") {
                        let ref_name = m.get("ref").unwrap().clone();
                        if group_tag_rules.contains_key(&ref_name) {
                            let group_rule = group_tag_rules.get(&ref_name).unwrap();
                            for r in group_rule.required_elements.iter() {
                                current_rule
                                    .as_mut()
                                    .unwrap()
                                    .required_elements
                                    .insert(r.0.clone(), r.1.clone());
                            }
                            for r in group_rule.optional_elements.iter() {
                                current_rule
                                    .as_mut()
                                    .unwrap()
                                    .optional_elements
                                    .insert(r.0.clone(), r.1.clone());
                            }
                            for r in group_rule.unique_elements.iter() {
                                current_rule
                                    .as_mut()
                                    .unwrap()
                                    .unique_elements
                                    .insert(r.0.clone(), r.1.clone());
                            }
                        } else {
                            return Err(anyhow::anyhow!("not found group name {}", &ref_name));
                        }
                    }
                } else if name.local_name.eq("attribute") {
                    // 属性
                    if current_rule.is_some() {
                        let m = attrs_to_map(&attributes);
                        if m.contains_key("use") {
                            current_rule.as_mut().unwrap().required_attrs.insert(
                                m.get("name").unwrap().clone(),
                                m.get("type").unwrap().clone(),
                            );
                        } else {
                            current_rule.as_mut().unwrap().optional_attrs.insert(
                                m.get("name").unwrap().clone(),
                                m.get("type").unwrap().clone(),
                            );
                        }
                    }
                } else if name.local_name.eq("element") {
                    if current_rule.is_none() {
                        continue;
                    }
                    // 子节点
                    let rule = current_rule.as_mut().unwrap();
                    if uniquqe_element {
                        rule.unique_elements.insert(
                            m.get("name").unwrap().clone(),
                            m.get("type").unwrap().clone(),
                        );
                    } else if sequence_element || m.contains_key("minOccurs") {
                        rule.optional_elements.insert(
                            m.get("name").unwrap().clone(),
                            m.get("type").unwrap().clone(),
                        );
                    } else if all_element {
                        rule.required_elements.insert(
                            m.get("name").unwrap().clone(),
                            m.get("type").unwrap().clone(),
                        );
                    }
                } else if name.local_name.eq("all") {
                    // required element
                    all_element = true;
                } else if name.local_name.eq("sequence") {
                    // optional element
                    sequence_element = true;
                } else if name.local_name.eq("choice") {
                    // only one
                    uniquqe_element = true;
                }
            }
            XmlEvent::EndElement { name } => {
                _level -= 1;
                if name.local_name.eq("complexType") {
                    if current_rule.is_some() {
                        let r = current_rule.take().unwrap();
                        result.insert(r.tag.clone(), r);
                        current_rule = None;
                    }
                } else if name.local_name.eq("group") {
                    group_level -= 1;
                    if is_group_rule && group_level == 0 {
                        is_group_rule = false;
                        if current_rule.is_some() {
                            let r = current_rule.take().unwrap();
                            group_tag_rules.insert(r.tag.clone(), r);
                            current_rule = None;
                        }
                    }
                } else if name.local_name.eq("all") {
                    // required element
                    all_element = false;
                } else if name.local_name.eq("sequence") {
                    // optional element
                    sequence_element = false;
                } else if name.local_name.eq("choice") {
                    // only one
                    uniquqe_element = false;
                }
            }
            XmlEvent::Characters(_) => {}
            _ => {}
        }
    }

    return Ok(result);
}

pub fn validate_scenario(
    tag_type: &str, root: Rc<RefCell<ContextNode>>, rules: &HashMap<String, Rule>,
) -> anyhow::Result<Vec<String>> {
    // debug
    // if tag_type.starts_with("OverrideControllerValueAction") {
    //     print!("");
    // }

    let mut errs = Vec::new();
    let borrow = root.borrow();
    let rule = if let Some(rule) = rules.get(tag_type) {
        rule
    } else if let Some(rule) = rules.get(&borrow.name) {
        rule
    } else {
        return Err(anyhow::anyhow!(
            "no rule for tag {} with parent {}",
            &borrow.name,
            &borrow.parent.as_ref().unwrap().borrow().name
        ));
    };

    // 检查必备的 attributes 是否都有
    for require_attr in rule.required_attrs.iter() {
        if !borrow.attributes.contains_key(require_attr.0) {
            errs.push(format!(
                "type {}/标签 {} 缺少attribute {}",
                rule.tag, &borrow.name, require_attr.0
            ));
        }
    }
    // 检查 attr 类型对不对
    for (attr, value) in borrow.attributes.iter() {
        if rule.required_attrs.contains_key(attr) {
            let r#type = rule.required_attrs.get(attr).unwrap();
            if !front_str_to_type(value, r#type) {
                errs.push(format!(
                    "标签:{} 的属性:{} 值:{} 类型不对{}",
                    rule.tag, attr, value, r#type
                ));
            }
        } else if rule.optional_attrs.contains_key(attr) {
            let r#type = rule.optional_attrs.get(attr).unwrap();
            if !front_str_to_type(value, r#type) {
                errs.push(format!(
                    "标签:{} 的属性:{} 值:{} 类型不对{}",
                    rule.tag, attr, value, r#type
                ));
            }
        } else {
            errs.push(format!(
                "type {}/标签 {} 有不合法attribute {}",
                rule.tag, &borrow.name, attr
            ));
        }
    }
    // 检查必备的 elements
    let child_names: HashSet<String> =
        borrow.child.clone().iter().map(|v| v.borrow().name.clone()).collect();
    for (name, _type) in rule.required_elements.iter() {
        if !child_names.contains(name) {
            errs.push(format!(
                "type {}/标签 {} 缺少element {}",
                rule.tag, &borrow.name, name
            ));
        }
    }
    // 检查 唯一的 element
    if rule.unique_elements.len() == 1 {
        if child_names.len() > 1 {
            errs.push(format!(
                "type {}/标签 {} 有多余element {:#?}",
                rule.tag, &borrow.name, child_names
            ));
        } else if child_names.is_empty() {
            errs.push(format!(
                "type {}/标签 {} 缺少element {:#?}",
                rule.tag, &borrow.name, child_names
            ));
        } else {
            let child_name = borrow.child[0].borrow().name.clone();
            if !rule.unique_elements.contains_key(&child_name) {
                errs.push(format!(
                    "type {}/标签 {} 缺少element {:#?}",
                    rule.tag, &borrow.name, child_names
                ));
            }
        }
    }

    for child in borrow.child.clone() {
        let child_name = child.borrow().name.clone();
        let child_type = if rule.required_elements.contains_key(&child_name) {
            rule.required_elements.get(&child_name).unwrap().clone()
        } else if rule.optional_elements.contains_key(&child_name) {
            rule.optional_elements.get(&child_name).unwrap().clone()
        } else if rule.unique_elements.contains_key(&child_name) {
            rule.unique_elements.get(&child_name).unwrap().clone()
        } else {
            "".to_string()
        };
        errs.append(&mut validate_scenario(&child_type, child, rules)?);
    }

    return Ok(errs);
}

fn front_str_to_type(value: &str, r#type: &str) -> bool {
    if r#type == "Double" {
        return value.parse::<f64>().is_ok();
    } else if r#type == "Boolean" {
        return value.to_lowercase().parse::<bool>().is_ok();
    } else if r#type == "Integer" {
        return value.parse::<i32>().is_ok();
    } else if r#type == "UnsignedShort" {
        return value.parse::<u32>().is_ok();
    }
    return true;
}
