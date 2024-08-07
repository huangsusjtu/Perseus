/// https://publications.pages.asam.net/standards/ASAM_OpenDRIVE/ASAM_OpenDRIVE_Specification/latest/specification/06_general_architecture/06_04_header.html
/// finished
/// need test
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Header {
    // attribute start
    #[serde(rename = "@revMajor")]
    pub major_version: u32,
    #[serde(rename = "@revMinor")]
    pub minor_version: u32,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    #[serde(rename = "@east", skip_serializing_if = "Option::is_none")]
    pub east: Option<f64>,
    #[serde(rename = "@north", skip_serializing_if = "Option::is_none")]
    pub north: Option<f64>,
    #[serde(rename = "@south", skip_serializing_if = "Option::is_none")]
    pub south: Option<f64>,
    #[serde(rename = "@west", skip_serializing_if = "Option::is_none")]
    pub west: Option<f64>,

    #[serde(rename = "@vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    // attribute end

    // children element start
    #[serde(rename = "@geoReference", skip_serializing_if = "Option::is_none")]
    pub geo_reference: Option<header::GeoReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<header::Offset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<header::License>,

    #[serde(rename = "defaultRegulations", skip_serializing_if = "Option::is_none")]
    pub default_regulations: Option<header::DefaultRegulations>,
    // children element end
}

pub mod header {
    use serde::{Deserialize, Serialize};

    use crate::opendrive::signal::signal;

    /// https://publications.pages.asam.net/standards/ASAM_OpenDRIVE/ASAM_OpenDRIVE_Specification/latest/specification/08_coordinate_systems/08_05_geo_referencing.html#top-3535a746-e0af-4020-b71c-3a94e7a855a1
    // example
    // <geoReference>
    //     <![CDATA[+proj=utm +zone=32 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0
    // +units=m +no_defs]]> </geoReference>
    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct GeoReference {
        #[serde(rename = "$text")]
        proj: String,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Offset {
        #[serde(rename = "@hdg")]
        pub hdg: f64,
        #[serde(rename = "@x")]
        pub x: f64,
        #[serde(rename = "@y")]
        pub y: f64,
        #[serde(rename = "@z")]
        pub z: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct License {
        #[serde(rename = "@name")]
        pub name: String,

        #[serde(rename = "@resource", skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(rename = "@spdxid", skip_serializing_if = "Option::is_none")]
        pub spdxid: Option<String>,
        #[serde(rename = "@text", skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct DefaultRegulations {
        #[serde(rename = "roadRegulations", skip_serializing_if = "Option::is_none")]
        pub road_regulations: Option<Vec<RoadRegulation>>,

        #[serde(rename = "signalRegulations", skip_serializing_if = "Option::is_none")]
        pub signal_regulations: Option<Vec<SignalRegulation>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct RoadRegulation {
        #[serde(rename = "@type")]
        pub r#type: String,

        pub semantics: signal::Semantics,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct SignalRegulation {
        #[serde(rename = "@type")]
        pub r#type: String,

        pub semantics: signal::Semantics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_header1() {
        let header = Header {
            major_version: 0,
            minor_version: 0,
            date: None,
            east: None,
            name: None,
            north: None,
            south: None,
            vendor: None,
            version: None,
            west: None,
            offset: None,
            license: None,
            geo_reference: None,
            default_regulations: None,
        };
        let xml = quick_xml::se::to_string(&header).unwrap();
        println!("{:#?}", xml);
    }

    #[test]
    fn it_works_header2() {
        let header = Header::default();
        let xml = quick_xml::se::to_string(&header).unwrap();
        println!("{:#?}", xml);
        // assert_eq!(
        //     "<Pose rotation_format=\"euler_rpy\" degrees=\"false\">0 0 0 0 0
        // 0</Pose>",     xml
        // );

        let header1: Header = quick_xml::de::from_str(&xml).unwrap();
        // assert_eq!(header1, header);
    }

    #[test]
    fn it_works_header3() {
        let xml = r#"<header name="myroad" revMajor="1" revMinor="5" date="2023-02-17 10:24:05.147309" north="0.0" south="0.0" east="0.0" west="0.0" />"#;
        let header1: Header = quick_xml::de::from_str(&xml).unwrap();
    }
}
