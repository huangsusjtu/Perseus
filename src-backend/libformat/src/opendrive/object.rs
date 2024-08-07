use serde::{Deserialize, Serialize};

use crate::opendrive::common::core::TYesNo;
use crate::opendrive::lane::lane::TBool;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Objects {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Vec<Object>>,
    #[serde(rename = "objectReference", skip_serializing_if = "Option::is_none")]
    pub object_reference: Option<object::ObjectReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<object::Tunnel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<object::Bridge>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Object {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@perpToRoad", skip_serializing_if = "Option::is_none")]
    pub perp_to_road: Option<TBool>,
    #[serde(rename = "@s")]
    pub s: f64,
    #[serde(rename = "@t")]
    pub t: f64,
    #[serde(rename = "@zOffset")]
    pub z_offset: f64,

    #[serde(rename = "@dynamic", skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<TYesNo>,
    #[serde(rename = "@hdg", skip_serializing_if = "Option::is_none")]
    pub hdg: Option<f64>,
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(rename = "@length", skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<object::EOrientation>,
    #[serde(rename = "@pitch", skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,
    #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(rename = "@roll", skip_serializing_if = "Option::is_none")]
    pub roll: Option<f64>,
    #[serde(rename = "@subtype", skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<object::EObjectType>,
    #[serde(rename = "@validLength", skip_serializing_if = "Option::is_none")]
    pub valid_length: Option<f64>,
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    // children
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<object::Repeat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline: Option<object::Outline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlines: Option<object::Outlines>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<object::Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<crate::opendrive::common::core::Validity>,
    #[serde(rename = "parkingSpace", skip_serializing_if = "Option::is_none")]
    pub parking_space: Option<object::ParkingSpace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markings: Option<object::Markings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<object::Borders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<object::Surface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeleton: Option<object::Skeleton>,
}

pub mod object {
    use serde::{Deserialize, Serialize};

    use crate::opendrive::lane::lane::{ELaneType, ERoadMarkColor, ERoadMarkWeight, TBool};

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Repeat {
        #[serde(rename = "@distance")]
        pub distance: f64,
        #[serde(rename = "@heightStart")]
        pub height_start: f64,
        #[serde(rename = "@heightEnd")]
        pub height_end: f64,
        #[serde(rename = "@length")]
        pub length: f64,
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@tStart")]
        pub t_start: f64,
        #[serde(rename = "@tEnd")]
        pub t_end: f64,
        #[serde(rename = "@zOffsetStart")]
        pub z_offset_start: f64,
        #[serde(rename = "@zOffsetEnd")]
        pub z_offset_end: f64,

        #[serde(
            rename = "@detachFromReferenceLine",
            skip_serializing_if = "Option::is_none"
        )]
        pub detach_from_reference_line: Option<TBool>,
        #[serde(rename = "@lengthStart", skip_serializing_if = "Option::is_none")]
        pub length_start: Option<f64>,
        #[serde(rename = "@lengthEnd", skip_serializing_if = "Option::is_none")]
        pub length_end: Option<f64>,
        #[serde(rename = "@radiusStart", skip_serializing_if = "Option::is_none")]
        pub radius_start: Option<f64>,
        #[serde(rename = "@radiusEnd", skip_serializing_if = "Option::is_none")]
        pub radius_end: Option<f64>,
        #[serde(rename = "@widthStart", skip_serializing_if = "Option::is_none")]
        pub width_start: Option<f64>,
        #[serde(rename = "@widthEnd", skip_serializing_if = "Option::is_none")]
        pub width_end: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Outline {
        #[serde(rename = "@closed", skip_serializing_if = "Option::is_none")]
        pub closed: Option<TBool>,
        #[serde(rename = "@fillType", skip_serializing_if = "Option::is_none")]
        pub fill_type: Option<EOutlineFillType>,
        #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
        pub id: Option<u32>,
        #[serde(rename = "@laneType", skip_serializing_if = "Option::is_none")]
        pub lane_type: Option<ELaneType>,
        #[serde(rename = "@outer", skip_serializing_if = "Option::is_none")]
        pub outer: Option<TBool>,

        #[serde(rename = "CornerRoad", skip_serializing_if = "Option::is_none")]
        pub corner_road: Option<Vec<CornerLocal>>,
        #[serde(rename = "cornerLocal", skip_serializing_if = "Option::is_none")]
        pub corner_local: Option<Vec<CornerLocal>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Outlines {
        pub outline: Vec<Outline>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct CornerRoad {
        #[serde(rename = "@dz")]
        pub dz: f64,
        #[serde(rename = "@height")]
        pub height: f64,
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@t")]
        pub t: f64,
        #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
        pub id: Option<u32>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct CornerLocal {
        #[serde(rename = "@height")]
        pub height: f64,
        #[serde(rename = "@u")]
        pub u: f64,
        #[serde(rename = "@v")]
        pub v: f64,
        #[serde(rename = "@z")]
        pub z: f64,
        #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
        pub id: Option<u32>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Material {
        #[serde(rename = "@friction", skip_serializing_if = "Option::is_none")]
        pub friction: Option<f64>,
        #[serde(rename = "@roadMarkColor", skip_serializing_if = "Option::is_none")]
        pub road_mark_color: Option<ERoadMarkColor>,

        #[serde(rename = "@roughness", skip_serializing_if = "Option::is_none")]
        pub roughness: Option<f64>,
        #[serde(rename = "@surface", skip_serializing_if = "Option::is_none")]
        pub surface: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct ParkingSpace {
        #[serde(rename = "@access")]
        pub access: ERoadObjectsObjectParkingSpaceAccess,
        #[serde(rename = "@restrictions", skip_serializing_if = "Option::is_none")]
        pub restrictions: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Markings {
        pub marking: Vec<Marking>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Marking {
        #[serde(rename = "@color")]
        pub color: ERoadMarkColor,
        #[serde(rename = "@lineLength")]
        pub line_length: f64,
        #[serde(rename = "@spaceLength")]
        pub space_length: f64,
        #[serde(rename = "@startOffset")]
        pub start_offset: f64,
        #[serde(rename = "@stopOffset")]
        pub stop_offset: f64,

        #[serde(rename = "@side", skip_serializing_if = "Option::is_none")]
        pub side: Option<ESideType>,
        #[serde(rename = "@weight", skip_serializing_if = "Option::is_none")]
        pub weight: Option<ERoadMarkWeight>,
        #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
        pub width: Option<f64>,
        #[serde(rename = "@zOffset", skip_serializing_if = "Option::is_none")]
        pub z_offset: Option<f64>,

        #[serde(rename = "cornerReference", skip_serializing_if = "Option::is_none")]
        pub corner_reference: Option<Vec<CornerReference>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct CornerReference {
        #[serde(rename = "@id")]
        pub id: u32,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Borders {
        pub border: Vec<Border>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Border {
        #[serde(rename = "@outlineId")]
        pub outline_id: u32,
        #[serde(rename = "@type")]
        pub r#type: EBorderType,
        #[serde(rename = "@width")]
        pub width: f64,

        #[serde(rename = "useCompleteOutline", skip_serializing_if = "Option::is_none")]
        pub use_complete_outline: Option<TBool>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Surface {
        #[serde(rename = "@CRG", skip_serializing_if = "Option::is_none")]
        pub crg: Option<CRG>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct CRG {
        #[serde(rename = "@file")]
        pub file: String,
        #[serde(rename = "@hideRoadSurfaceCRG")]
        pub hide_road_surface_crg: TBool,
        #[serde(rename = "@zScale")]
        pub z_scale: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Skeleton {
        pub polyline: Vec<Polyline>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Polyline {
        #[serde(rename = "@id")]
        pub id: u32,

        #[serde(rename = "vertexRoad", skip_serializing_if = "Option::is_none")]
        pub vertex_road: Option<Vec<VertexRoad>>,
        #[serde(rename = "vertexLocal", skip_serializing_if = "Option::is_none")]
        pub vertex_local: Option<Vec<VertexLocal>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct VertexRoad {
        #[serde(rename = "@dz")]
        pub dz: f64,
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@t")]
        pub t: f64,

        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        pub id: Option<u32>,
        #[serde(rename = "intersectionPoint", skip_serializing_if = "Option::is_none")]
        pub intersection_point: Option<TBool>,
        #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
        pub radius: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct VertexLocal {
        #[serde(rename = "@u")]
        pub u: f64,
        #[serde(rename = "@v")]
        pub v: f64,
        #[serde(rename = "@z")]
        pub z: f64,

        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        pub id: Option<u32>,
        #[serde(rename = "intersectionPoint", skip_serializing_if = "Option::is_none")]
        pub intersection_point: Option<TBool>,
        #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
        pub radius: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct ObjectReference {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@t")]
        pub t: f64,
        #[serde(rename = "@orientation")]
        pub orientation: EOrientation,

        #[serde(rename = "@validLength", skip_serializing_if = "Option::is_none")]
        pub valid_length: Option<f64>,
        #[serde(rename = "@zOffset", skip_serializing_if = "Option::is_none")]
        pub z_offset: Option<f64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub validity: Option<crate::opendrive::common::core::Validity>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Tunnel {
        #[serde(rename = "@type")]
        pub r#type: ETunnelType,
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@length")]
        pub length: f64,
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@lighting", skip_serializing_if = "Option::is_none")]
        pub lighting: Option<f64>,
        #[serde(rename = "@daylight", skip_serializing_if = "Option::is_none")]
        pub daylight: Option<f64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub validity: Option<crate::opendrive::common::core::Validity>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Bridge {
        #[serde(rename = "@type")]
        pub r#type: EBridgeType,
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@length")]
        pub length: f64,
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub validity: Option<crate::opendrive::common::core::Validity>,
    }

    // list enum
    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ETunnelType {
        #[default]
        Standard,
        Underpass,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum EBorderType {
        #[default]
        Concrete,
        Curb,
        Paint,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ESideType {
        #[default]
        Front,
        Left,
        Rear,
        Right,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum EOutlineFillType {
        #[default]
        Asphalt,
        Cobble,
        Concrete,
        Grass,
        Gravel,
        Paint,
        Pavement,
        Soil,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum EObjectType {
        Barrier,
        Bike,
        Building,
        Bus,
        Car,
        Crosswalk,
        Gantry,
        Motorbike,
        #[default]
        None,
        Obstacle,
        ParkingSpace,
        Patch,
        Pedestrian,
        Pole,
        Railing,
        RoadMark,
        RoadSurface,
        SoundBarrier,
        StreetLamp,
        TrafficIsland,
        Trailer,
        Train,
        Tram,
        Tree,
        Van,
        Vegetation,
        Wind,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum EBridgeType {
        #[default]
        Brick,
        Concrete,
        Steel,
        Wood,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EOrientation {
        #[serde(rename = "+")]
        Plus,
        #[serde(rename = "-")]
        Minus,
        #[default]
        #[serde(rename = "none")]
        None,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ERoadObjectsObjectParkingSpaceAccess {
        #[default]
        All,
        Bus,
        Car,
        Electric,
        Handicapped,
        Residents,
        Truck,
        Women,
    }
}

#[cfg(test)]
mod tests {
    use crate::opendrive::road::Road;

    use super::*;

    #[test]
    fn it_works1() {
        let xml = r#"
        <objects>
			<object type="pole" name="pole" id="0" s="0.0" t="3.57" zOffset="-0.2" validLength="0.0" orientation="+" length="0.06" width="0.06" radius="0.5" height="2.35" hdg="0.0" pitch="0.0" roll="0.0">
				<validity fromLane="-3" toLane="3" />
			</object>
			<object type="pole" name="pole" id="1" s="0.0" t="-3.57" zOffset="-0.2" validLength="0.0" orientation="+" length="0.06" width="0.06" radius="0.5" height="2.35" hdg="0.0" pitch="0.0" roll="0.0" />
			<object type="pole" name="pole" id="2" s="100.0" t="3.57" zOffset="-0.2" validLength="0.0" orientation="+" length="0.06" width="0.06" radius="0.5" height="2.35" hdg="0.0" pitch="0.0" roll="0.0" />
			<object type="pole" name="pole" id="3" s="100.0" t="-3.57" zOffset="-0.2" validLength="0.0" orientation="+" length="0.06" width="0.06" radius="0.5" height="2.35" hdg="0.0" pitch="0.0" roll="0.0" />
			<object type="pole" name="pole" id="4" s="200.0" t="3.57" zOffset="-0.2" validLength="0.0" orientation="+" length="0.06" width="0.06" radius="0.5" height="2.35" hdg="0.0" pitch="0.0" roll="0.0" />
			<object type="pole" name="pole" id="5" s="200.0" t="-3.57" zOffset="-0.2" validLength="0.0" orientation="+" length="0.06" width="0.06" radius="0.5" height="2.35" hdg="0.0" pitch="0.0" roll="0.0" />
			<object type="pole" name="pole" id="6" s="500.0" t="3.57" zOffset="-0.2" validLength="0.0" orientation="+" length="0.06" width="0.06" radius="0.5" height="2.35" hdg="0.0" pitch="0.0" roll="0.0" />
			<object type="pole" name="pole" id="7" s="500.0" t="-3.57" zOffset="-0.2" validLength="0.0" orientation="+" length="0.06" width="0.06" radius="0.5" height="2.35" hdg="0.0" pitch="0.0" roll="0.0" />
		</objects>
        "#;
        let r = quick_xml::de::from_str::<Objects>(&xml).unwrap();
        println!("{:#?}", r);
    }

    #[test]
    fn it_works2() {
        let r = Road::default();
        let xml = quick_xml::se::to_string(&r).unwrap();
        println!("{:#?}", xml);
    }
}
