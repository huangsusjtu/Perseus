use serde::{Deserialize, Serialize};

use crate::opendrive::common;
use crate::opendrive::lane::Lanes;
use crate::opendrive::object::Objects;

/// https://publications.pages.asam.net/standards/ASAM_OpenDRIVE/ASAM_OpenDRIVE_Specification/latest/specification/10_roads/10_01_introduction.html#top-f0ae72f0-300e-4f8b-9c9b-7f68a467a9f7
/// finish
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Road {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@junction")]
    pub junction: String,
    #[serde(rename = "@length")]
    pub length: f64,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@rule", skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>, // LHT  or RHT

    //
    #[serde(rename = "planView")]
    pub plan_view: common::planview::PlanView, // 这个必须的；记录road的中心线坐标的
    // <Lanes>
    pub lanes: Lanes, // only one

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Vec<road::RoadLink>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<road::RoadType>>,
    #[serde(rename = "elevationProfile", skip_serializing_if = "Option::is_none")]
    pub elevation_profile: Option<road::ElevationProfile>,
    #[serde(rename = "lateralProfile", skip_serializing_if = "Option::is_none")]
    pub lateral_profile: Option<road::LateralProfile>,

    // <Objects>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<Objects>>,
}

pub mod road {
    use serde::{Deserialize, Serialize};

    use crate::opendrive::common::Speed;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct RoadType {
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@type")]
        pub r#type: ERoadType,

        #[serde(rename = "@country", skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,

        // element
        #[serde(skip_serializing_if = "Option::is_none")]
        pub speed: Option<Speed>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct RoadLink {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub predecessor: Option<Predecessor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub successor: Option<Successor>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Predecessor {
        #[serde(rename = "@elementId")]
        pub element_id: String,
        #[serde(rename = "@elementType")]
        pub element_type: String, // junction or road

        // Contact point of link on the linked element\
        // option value : start or end
        #[serde(rename = "contactPoint", skip_serializing_if = "Option::is_none")]
        pub contact_point: Option<String>,
        // To be provided when elementS is used for the connection definition.
        // Indicates the direction on the predecessor from which the
        // road is entered. option value : + or -
        #[serde(rename = "elementDir", skip_serializing_if = "Option::is_none")]
        pub element_dir: Option<String>,

        // Alternative to contactPoint for virtual junctions. Indicates a
        // connection within the predecessor, meaning not at the start
        // or end of the predecessor. Shall only be used for
        // elementType "road"
        #[serde(rename = "elementS", skip_serializing_if = "Option::is_none")]
        pub element_s: Option<f64>, // m
    }

    pub type Successor = Predecessor;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct ElevationProfile {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub elevation: Option<Vec<Elevation>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Elevation {
        #[serde(rename = "@a")]
        pub a: f64,
        #[serde(rename = "@b")]
        pub b: f64,
        #[serde(rename = "@c")]
        pub c: f64,
        #[serde(rename = "@d")]
        pub d: f64,
        #[serde(rename = "@s")]
        pub s: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct LateralProfile {
        #[serde(rename = "superelevation", skip_serializing_if = "Option::is_none")]
        pub super_elevation: Option<Vec<Superelevation>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shape: Option<Shape>,
        #[serde(
            rename = "crossSectionSurface",
            skip_serializing_if = "Option::is_none"
        )]
        pub cross_section_surface: Option<CrossSectionSurface>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Superelevation {
        #[serde(rename = "@a")]
        pub a: f64,
        #[serde(rename = "@b")]
        pub b: f64,
        #[serde(rename = "@c")]
        pub c: f64,
        #[serde(rename = "@d")]
        pub d: f64,
        #[serde(rename = "@s")]
        pub s: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Shape {
        // Polynom parameter a, relative height at @t (dt=0)
        #[serde(rename = "@a")]
        pub a: f64,
        // Polynom parameter b
        #[serde(rename = "@b")]
        pub b: f64,
        // Polynom parameter c
        #[serde(rename = "@c")]
        pub c: f64,
        // Polynom parameter d
        #[serde(rename = "@d")]
        pub d: f64,
        // s-coordinate of start position
        #[serde(rename = "@s")]
        pub s: f64,
        // t-coordinate of start position
        #[serde(rename = "@t")]
        pub t: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct CrossSectionSurface {
        #[serde(rename = "tOffset", skip_serializing_if = "Option::is_none")]
        pub t_offset: Option<TOffset>,

        #[serde(rename = "surfaceStrips", skip_serializing_if = "Option::is_none")]
        pub surface_strips: Option<SurfaceStrips>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct TOffset {
        #[serde(rename = "coefficients", skip_serializing_if = "Option::is_none")]
        pub coefficients: Option<Vec<Coefficients>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Coefficients {
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@a")]
        pub a: f64,
        #[serde(rename = "@b")]
        pub b: f64,
        #[serde(rename = "@c")]
        pub c: f64,
        #[serde(rename = "@d")]
        pub d: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct SurfaceStrips {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub strip: Option<Vec<Strip>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Strip {
        #[serde(rename = "@id")]
        pub id: i32,
        #[serde(rename = "@mode")]
        pub mode: Option<EStripMode>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub width: Option<CoefficientsVec>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub constant: Option<CoefficientsVec>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub linear: Option<CoefficientsVec>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quadratic: Option<CoefficientsVec>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cubic: Option<CoefficientsVec>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct CoefficientsVec {
        pub coefficients: Vec<Coefficients>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum ERoadLinkElementType {
        Junction,
        #[default]
        Road,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum EParamPoly3pRange {
        #[default]
        Normalized,
        ArcLength,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum ERoadType {
        Bicycle,
        #[default]
        LowSpeed,
        Motorway,
        Pedestrian,
        Rural,
        TownArterial,
        TownCollector,
        TownExpressway,
        TownLocal,
        TownPlayStreet,
        TownPrivate,
        Town,
        Unknown,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum EStripMode {
        #[default]
        Independent,
        Relative,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EMaxSpeedString {
        #[default]
        #[serde(rename = "no limit")]
        NoLimit,
        #[serde(rename = "undefined")]
        Undefined,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum ETrafficRule {
        #[default]
        #[serde(rename = "LHT")]
        LHT,
        #[serde(rename = "RHT")]
        RHT,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum EDirection {
        #[default]
        Opposite,
        Same,
    }

    // #[derive(Deserialize, Serialize, Debug, Default)]
    // #[serde(rename_all = "cameCase")]
    // pub enum ECountryCodeDeprecated {
    //     Austria,
    //     Brazil,
    //     #[default]
    //     China,
    //     France,
    //     Germany,
    //     Italy,
    //     OpenDRIVE,
    //     Switzerland,
    //     USA
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let r = road::RoadType::default();
        let xml = quick_xml::se::to_string(&r).unwrap();
        println!("{:#?}", xml);
    }

    #[test]
    fn it_works2() {
        let r = Road::default();
        let xml = quick_xml::se::to_string(&r).unwrap();
        println!("{:#?}", xml);
    }

    #[test]
    fn it_works3() {
        let xml = r#"
                        <road rule="RHT" id="4" junction="-1" length="50">
                            <link>
                                <successor elementType="junction" elementId="1"/>
                                <predecessor elementType="road" elementId="3" contactPoint="end"/>
                            </link>
                            <planView>
                                <geometry s="0" x="0" y="0" hdg="0" length="50">
                                    <line/>
                                </geometry>
                            </planView>
                            <elevationProfile/>
                            <lateralProfile/>
                            <lanes>
                                <laneSection s="0">
                                    <left>
                                        <lane id="1" type="driving" level="false">
                                            <link>
                                                <predecessor id="1"/>
                                            </link>
                                            <width a="3" b="0" c="0" d="0" sOffset="0"/>
                                            <roadMark sOffset="0" type="solid" weight="standard" color="standard" height="0.02" width="0.2"/>
                                        </lane>
                                    </left>
                                    <center>
                                        <lane id="0" type="none" level="false">
                                            <roadMark sOffset="0" type="broken" weight="standard" color="standard" height="0.02" width="0.2"/>
                                        </lane>
                                    </center>
                                    <right>
                                        <lane id="-1" type="driving" level="false">
                                            <link>
                                                <predecessor id="-1"/>
                                            </link>
                                            <width a="3" b="0" c="0" d="0" sOffset="0"/>
                                            <roadMark sOffset="0" type="solid" weight="standard" color="standard" height="0.02" width="0.2"/>
                                        </lane>
                                    </right>
                                </laneSection>
                            </lanes>
                        </road>
        "#;
        let r = quick_xml::de::from_str::<Road>(&xml).unwrap();
        println!("{:#?}", xml);
    }

    #[test]
    fn it_works4() {
        let xml = r#"
            <lanes>
                <laneSection s="0">
                    <left>
                        <lane id="1" type="driving" level="false">
                            <link>
                                <predecessor id="1"/>
                            </link>
                            <width a="3" b="0" c="0" d="0" sOffset="0"/>
                            <roadMark sOffset="0" type="solid" weight="standard" color="standard" height="0.02" width="0.2"/>
                        </lane>
                    </left>
                    <center>
                        <lane id="0" type="none" level="false">
                            <roadMark sOffset="0" type="broken" weight="standard" color="standard" height="0.02" width="0.2"/>
                        </lane>
                    </center>
                    <right>
                        <lane id="-1" type="driving" level="false">
                            <link>
                                <predecessor id="-1"/>
                            </link>
                            <width a="3" b="0" c="0" d="0" sOffset="0"/>
                            <roadMark sOffset="0" type="solid" weight="standard" color="standard" height="0.02" width="0.2"/>
                        </lane>
                    </right>
                </laneSection>
            </lanes>
        "#;
        let r = quick_xml::de::from_str::<Lanes>(&xml).unwrap();
        println!("{:#?}", xml);
    }

    #[test]
    fn it_works5() {
        let xml = r#"
             <road rule="RHT" id="100" junction="1" length="33.205298710624206">
                <link>
                    <predecessor elementType="road" elementId="4" contactPoint="end" />
                    <successor elementType="road" elementId="5" contactPoint="start" />
                </link>
                <planView>
                    <geometry s="0" x="50.0" y="0.0" hdg="0" length="10.471384127159908">
                        <spiral curvStart="1e-09" curvEnd="-0.06909484644623465" />
                    </geometry>
                    <geometry s="10.471384127159908" x="60.335173104070776" y="-1.2509524149051037" hdg="-0.3617593339371343" length="12.262530456304393">
                        <spiral curvStart="-0.06909484644623465" curvEnd="-0.06909484644623465" />
                    </geometry>
                    <geometry s="22.7339145834643" x="68.7490475850949" y="-9.664826895929222" hdg="-1.209036992857762" length="10.471384127159908">
                        <spiral curvStart="-0.06909484644623465" curvEnd="9.999999994736442e-10" />
                    </geometry>
                </planView>
                <elevationProfile />
                <lateralProfile />
                <lanes>
                    <laneSection s="0">
                        <left>
                            <lane id="1" type="driving" level="false">
                                <link>
                                    <predecessor id="1" />
                                    <successor id="1" />
                                </link>
                                <width a="3" b="0" c="0" d="0" sOffset="0" />
                            </lane>
                        </left>
                        <center>
                            <lane id="0" type="none" level="false" />
                        </center>
                        <right>
                            <lane id="-1" type="driving" level="false">
                                <link>
                                    <predecessor id="-1" />
                                    <successor id="-1" />
                                </link>
                                <width a="3" b="0" c="0" d="0" sOffset="0" />
                                <roadMark sOffset="0" type="solid" weight="standard" color="standard" height="0.02" width="0.2" />
                            </lane>
                        </right>
                    </laneSection>
                </lanes>
            </road>
            "#;
        let r = quick_xml::de::from_str::<Road>(&xml).unwrap();
        println!("{:#?}", xml);
    }

    #[test]
    fn it_works6() {
        let xml = r#"
            <road name="changingLaneWidth" length="50.0" id="0" junction="-1">
                <objects />
                <signals />
            </road>
            "#;
        let r = quick_xml::de::from_str::<Road>(&xml).unwrap();
        println!("{:#?}", xml);
    }
}
