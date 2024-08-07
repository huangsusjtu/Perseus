use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use anyhow::anyhow;
use xml::attribute::OwnedAttribute;
use xml::EventReader;
use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::openscenario::preprocess::attrs_to_map;

#[derive(Clone)]
pub(crate) enum Parameter {
    Integer(i32),
    String(String),
    Double(f64),
    Boolean(bool),
}

impl Into<String> for &Parameter {
    fn into(self) -> String {
        match self {
            Parameter::Integer(v) => v.to_string(),
            Parameter::String(v) => v.to_string(),
            Parameter::Double(v) => v.to_string(),
            Parameter::Boolean(v) => v.to_string(),
        }
    }
}

pub(crate) type ParameterDeclarations = HashMap<String, Parameter>;

pub(crate) struct ContextNode {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub content: Option<String>,

    pub start: XmlEvent,
    pub end: XmlEvent,

    pub parameter_declarations: ParameterDeclarations,

    pub child: Vec<Rc<RefCell<ContextNode>>>,
    pub parent: Option<Rc<RefCell<ContextNode>>>,
}

pub(crate) fn xml_to_context_tree(xml: &str) -> anyhow::Result<Rc<RefCell<ContextNode>>> {
    let mut root: Option<Rc<RefCell<ContextNode>>> = None;
    let mut current: Option<Rc<RefCell<ContextNode>>> = None;

    let parser = EventReader::new(xml.as_bytes());
    for e in parser {
        let event = e?;
        match event {
            XmlEvent::StartDocument { .. } => {}
            XmlEvent::EndDocument => {}
            XmlEvent::ProcessingInstruction { name, data } => {
                println!("ProcessingInstruction {}, {:?}", name, data);
            }
            XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            } => {
                let mut attributes_map = attrs_to_map(&attributes);
                if current.is_some() {
                    // 非root节点需要检测 attribute是不是要
                    // ParameterDeclaration过滤
                    let parameters = generate_parameters(current.clone().unwrap());
                    if name.local_name.eq("ParameterDeclaration") {
                        let (key, parameter) =
                            parse_tag_parameter_declaration(&attributes_map, &parameters)?;
                        current
                            .as_mut()
                            .unwrap()
                            .borrow_mut()
                            .parameter_declarations
                            .insert(key, parameter);
                    } else {
                        let mut update: HashMap<String, String> = HashMap::default();
                        for (k, v) in attributes_map.iter() {
                            if v.starts_with("$") {
                                let new_v =
                                    crate::openscenario::preprocess::expression::handler_expression(
                                        v,
                                        &parameters,
                                    )?;
                                update.insert(k.clone(), new_v);
                            }
                        }
                        for p in update {
                            attributes_map.insert(p.0, p.1);
                        }
                    }
                }

                let attr_vec = map_to_attr(&attributes_map);
                let node = ContextNode {
                    name: name.local_name.clone(),
                    attributes: attributes_map,
                    content: None,

                    start: XmlEvent::StartElement {
                        name: name.clone(),
                        attributes: attr_vec,
                        namespace,
                    },

                    end: XmlEvent::EndElement { name },
                    parameter_declarations: HashMap::new(),
                    child: vec![],
                    parent: current.clone(),
                };

                let child = Rc::new(RefCell::new(node));
                if root.is_none() {
                    // 第一个节点是root
                    root = Some(child.clone());
                }
                // 父子关系
                if current.is_some() {
                    current.as_mut().unwrap().borrow_mut().child.push(child.clone());
                }
                current = Some(child);
            }
            XmlEvent::EndElement { name } => {
                let mut parent = current.as_ref().unwrap().borrow().parent.clone();
                if name.local_name.eq("ParameterDeclarations") {
                    parent.as_mut().unwrap().borrow_mut().parameter_declarations =
                        current.as_ref().unwrap().borrow_mut().parameter_declarations.clone();
                }
                // 往上进一层
                current = parent;
            }
            XmlEvent::CData(data) => {
                println!("XmlEvent::CData = {}", data);
            }
            XmlEvent::Comment(_) => {}
            XmlEvent::Characters(data) => {
                if current.is_some() {
                    current.as_mut().unwrap().borrow_mut().content = Some(data.trim().to_string());
                }
            }
            XmlEvent::Whitespace(_) => {}
        }
    }

    Ok(root.unwrap())
}

pub(crate) fn context_tree_to_xml(root: Rc<RefCell<ContextNode>>) -> String {
    let mut xml: String = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string();
    context_tree_to_xml_inner(root.clone(), &mut xml);
    xml
}

fn context_tree_to_xml_inner(root: Rc<RefCell<ContextNode>>, xml: &mut String) {
    let borrow_m = root.borrow_mut();
    if borrow_m.name.eq("ParameterDeclarations") {
        return;
    }

    let event = borrow_m.start.clone();
    #[allow(unused_variables)]
    match event {
        XmlEvent::StartElement {
            name,
            attributes,
            namespace,
        } => {
            xml.push('<');
            xml.push_str(&name.local_name);
            for a in attributes {
                xml.push(' ');
                xml.push_str(&a.name.local_name);
                xml.push_str(" = ");
                xml.push('"');
                xml.push_str(&a.value);
                xml.push('"');
            }
            xml.push_str(">");
        }
        _ => {}
    }

    if borrow_m.content.is_none() {
        for child in borrow_m.child.clone() {
            context_tree_to_xml_inner(child, xml);
        }
    } else {
        xml.push_str(borrow_m.content.as_ref().unwrap());
    }

    let event = borrow_m.end.clone();
    match event {
        XmlEvent::EndElement { name } => {
            xml.push_str("</");
            xml.push_str(&name.local_name);
            xml.push_str(">");
        }
        _ => {}
    }
}

fn map_to_attr(maps: &HashMap<String, String>) -> Vec<OwnedAttribute> {
    maps.iter()
        .map(|v| OwnedAttribute {
            name: OwnedName {
                local_name: v.0.to_string(),
                namespace: None,
                prefix: None,
            },
            value: v.1.to_string(),
        })
        .collect()
}

fn parse_tag_parameter_declaration(
    attributes: &HashMap<String, String>, parameters: &ParameterDeclarations,
) -> anyhow::Result<(String, Parameter)> {
    let name = attributes.get("name").unwrap();
    let parameter_type = attributes.get("parameterType").unwrap();
    let value = attributes.get("value").unwrap().clone();
    let value = if value.starts_with("$") {
        crate::openscenario::preprocess::expression::handler_expression(
            attributes.get("value").unwrap(),
            parameters,
        )?
    } else {
        value
    };

    let parameter = if parameter_type.eq("string") || parameter_type.eq("dateTime") {
        Parameter::String(value)
    } else if parameter_type.eq("integer") || parameter_type.eq("int") {
        Parameter::Integer(value.parse::<i32>().unwrap())
    } else if parameter_type.eq("boolean") || parameter_type.eq("bool") {
        Parameter::Boolean(value.parse::<bool>().unwrap())
    } else if parameter_type.eq("double") {
        Parameter::Double(value.parse::<f64>().unwrap())
    } else {
        return Err(anyhow!("unsupported value type {}", parameter_type));
    };
    return Ok((name.clone(), parameter));
}

fn generate_parameters(current: Rc<RefCell<ContextNode>>) -> ParameterDeclarations {
    let mut parameters: ParameterDeclarations = ParameterDeclarations::default();
    let mut p = Some(current);
    while p.is_some() {
        for iter in p.as_ref().unwrap().borrow().parameter_declarations.iter() {
            if !parameters.contains_key(iter.0) {
                parameters.insert(iter.0.to_string(), iter.1.clone());
            }
        }
        p = p.unwrap().borrow().parent.clone();
    }

    return parameters;
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use crate::openscenario::preprocess::context_tree::{context_tree_to_xml, xml_to_context_tree};
    use crate::pretty_xml::prettify_xml;

    #[test]
    fn it_works1() {
        let xml = std::fs::read_to_string(
            std::env::current_dir()
                .unwrap()
                .join("asset")
                .join("openscenario")
                .join("ControllerCatalog.xosc"),
        )
            .unwrap();

        let r = xml_to_context_tree(&xml).unwrap();
        println!("{:#?}", 123);

        let mut xml = context_tree_to_xml(r);
        println!("{:#?}", xml);
        std::fs::write(
            std::env::current_dir().unwrap().join("test").join("ControllerCatalog.xosc"),
            &xml,
        );
        xml = prettify_xml(&xml);
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
            println!("Processing file: {:?}", file);
            let xml = fs::read_to_string(target_dir.join(file)).unwrap();
            let root_context = xml_to_context_tree(&xml).unwrap();
            let new_xml = context_tree_to_xml(root_context);
            fs::write(
                std::env::current_dir().unwrap().join("asset").join("new").join(file),
                new_xml,
            );
        }
    }
}
