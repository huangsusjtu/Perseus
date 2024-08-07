/// align to doc: http://sdformat.org/spec?ver=1.11&elem=light
/// finished
use serde::{Deserialize, Serialize};

use crate::sdf::Color;
use crate::sdf::util::{Pose, Vector3};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Light {
    // attribute start
    #[serde(rename = "@name")]
    pub name: String,
    // The light type: point, directional, spot.
    // Default: point
    #[serde(rename = "@type")]
    pub r#type: String,
    // attribute end

    // children element start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_shadows: Option<bool>, // Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_on: Option<bool>, // Default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualize: Option<bool>, // Default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intensity: Option<f64>, // Default: 1

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffuse: Option<Color>, // Default: 1 1 1 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular: Option<Color>,
    /* Default: 0.100000001 0.100000001
                                     * 0.100000001 1 */

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation: Option<Attenuation>, // Light attenuation

    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Vector3<f64>>, // Default: 0 0 -1

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot: Option<Spot>, // Spot light parameters

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    // children element end
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Attenuation {
    // Range of the light
    // Default: 10
    pub range: f64,

    // The linear attenuation factor: 1 means attenuate evenly over the
    // distance. Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<f64>,

    // The constant attenuation factor: 1.0 means never attenuate, 0.0 is
    // complete attenutation. Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<f64>,

    // The quadratic attenuation factor: adds a curvature to the attenuation.
    // Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadratic: Option<f64>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default)]
pub struct Spot {
    // Angle covered by the bright inner cone
    // Default: 0
    pub inner_angle: f64,

    // Angle covered by the outer cone
    // Default: 0
    pub outer_angle: f64,

    // The rate of falloff between the inner and outer cones. 1.0 means a
    // linear falloff, less means slower falloff, higher means faster
    // falloff. Default: 0
    pub falloff: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_spot1() {
        let spot = Spot {
            inner_angle: 0.0,
            outer_angle: 0.0,
            falloff: 0.0,
        };
        let xml = quick_xml::se::to_string(&spot).unwrap();

        println!("{:#?}", xml);
        assert_eq!(
            "<Spot><inner_angle>0</inner_angle><outer_angle>0</outer_angle><falloff>0</falloff></\
             Spot>",
            xml
        );
    }

    #[test]
    fn it_works_spot2() {
        let s = "<Spot><inner_angle>0</inner_angle><outer_angle>0</outer_angle><falloff>0</\
                 falloff></Spot>";
        let spot1 = Spot {
            inner_angle: 0.0,
            outer_angle: 0.0,
            falloff: 0.0,
        };
        let spot: Spot = quick_xml::de::from_str(s).unwrap();
        assert_eq!(spot, spot1);
    }

    #[test]
    fn it_works_attenuation1() {
        let spot = Attenuation {
            range: 10.0,
            linear: Some(1.0),
            constant: Some(1.0),
            quadratic: Some(0.0),
        };
        let xml = quick_xml::se::to_string(&spot).unwrap();

        println!("{:#?}", xml);
        assert_eq!(
            "<Attenuation><range>10</range><linear>1</linear><constant>1</constant><quadratic>0</\
             quadratic></Attenuation>",
            xml
        );
    }

    #[test]
    fn it_works_attenuation2() {
        let s = r#"
            <Attenuation>
                <range>10</range>
                <linear>1</linear>
                <constant>1</constant>
                <quadratic>0</quadratic>
            </Attenuation>
        "#;
        let spot1 = Attenuation {
            range: 10.0,
            linear: Some(1.0),
            constant: Some(1.0),
            quadratic: Some(0.0),
        };
        let spot: Attenuation = quick_xml::de::from_str(s).unwrap();
        assert_eq!(spot, spot1);
    }

    #[test]
    fn it_works_light1() {
        let s = Light {
            name: "1".to_string(),
            r#type: "point".to_string(),
            cast_shadows: Some(false),
            light_on: Some(true),
            visualize: Some(false),
            intensity: Some(1.0),
            diffuse: Some(Color::new(1f64, 1f64, 1f64, 1f64)),
            specular: Some(Color::new(0.1, 0.1, 0.1, 1f64)),
            attenuation: Some(Attenuation {
                range: 10.0,
                linear: Some(1.0),
                constant: Some(1.0),
                quadratic: Some(0.0),
            }),
            direction: Some(Vector3::new(0.0, 0.0, -1.0)),
            spot: Some(Spot::default()),
            pose: Some(Pose::default()),
        };
        let xml = quick_xml::se::to_string(&s).unwrap();
        println!("{:#?}", xml);
        assert_eq!(
            "<Light name=\"1\" \
             type=\"point\"><cast_shadows>false</cast_shadows><light_on>true</\
             light_on><visualize>false</visualize><intensity>1</intensity><diffuse>1 1 1 \
             1</diffuse><specular>0.1 0.1 0.1 \
             1</specular><attenuation><range>10</range><linear>1</linear><constant>1</\
             constant><quadratic>0</quadratic></attenuation><direction>0 0 \
             -1</direction><spot><inner_angle>0</inner_angle><outer_angle>0</\
             outer_angle><falloff>0</falloff></spot><pose rotation_format=\"euler_rpy\" \
             degrees=\"false\">0 0 0 0 0 0</pose></Light>",
            xml
        );
    }

    #[test]
    fn it_works_light2() {
        let s = r#"
            <Light name="1" type="point">
                        <cast_shadows>false</cast_shadows>
                <light_on>true</light_on>
                <visualize>false</visualize>
                <intensity>1</intensity>
                <diffuse>1 1 1 1</diffuse>
                <specular>0.1 0.1 0.1 1</specular>
                <attenuation>
                    <range>10</range>
                    <linear>1</linear>
                    <constant>1</constant>
                    <quadratic>0</quadratic>
                </attenuation>
                <direction>0 0 -1</direction>
                <spot>
                    <inner_angle>0</inner_angle>
                    <outer_angle>0</outer_angle>
                    <falloff>0</falloff>
                </spot>
                <pose>0 0 0 0 0 0</pose>
            </Light>
        "#;
        let l = Light {
            name: "1".to_string(),
            r#type: "point".to_string(),
            cast_shadows: Some(false),
            light_on: Some(true),
            visualize: Some(false),
            intensity: Some(1.0),
            diffuse: Some(Color::new(1f64, 1f64, 1f64, 1f64)),
            specular: Some(Color::new(0.1, 0.1, 0.1, 1f64)),
            attenuation: Some(Attenuation {
                range: 10.0,
                linear: Some(1.0),
                constant: Some(1.0),
                quadratic: Some(0.0),
            }),
            direction: Some(Vector3::new(0.0, 0.0, -1.0)),
            spot: Some(Spot::default()),
            pose: Some(Pose::default()),
        };
        let l1: Light = quick_xml::de::from_str(s).unwrap();
        // assert_eq!(l, l1);
    }
}
