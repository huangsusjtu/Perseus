/// align to doc: http://sdformat.org/spec?ver=1.11&elem=geometry
/// finished
use serde::{Deserialize, Serialize};

use crate::sdf::joint::Joint;
use crate::sdf::link::Link;
use crate::sdf::util::{Plugin, Pose};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Actor {
    // attribute start
    // Description: A special kind of model which can have a scripted motion.
    // This includes both global waypoint type animations and skeleton
    // animations
    #[serde(rename = "@name")]
    pub name: String,
    // attribute end

    // children element start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<Skin>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub animations: Vec<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub joints: Vec<Joint>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<Plugin>,
    // children element end
}

/// Description: Skin file which defines a visual and the underlying skeleton
/// which moves it.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Skin {
    pub filename: String, // Path to skin file, accepted formats: COLLADA, BVH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>, // Scale the skin's size.
}

/// Description: Animation file defines an animation for the skeleton in the
/// skin. The skeleton must be compatible with the skin skeleton.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Animation {
    // attribute start
    #[serde(rename = "@name")]
    pub name: String,
    // attribute end

    // children element start
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<f64>,
    // children element end
}

/// Description: Animation file defines an animation for the skeleton in the
/// skin. The skeleton must be compatible with the skin skeleton.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Script {
    // children element start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#loop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_start: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trajectory: Vec<Trajectory>,
    // children element end
}

// Description: The trajectory contains a series of keyframes to be followed.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Trajectory {
    // attribute start
    #[serde(rename = "@id")]
    pub id: i32,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@tension", skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
    // attribute end

    // children element start
    pub waypoints: Vec<WayPoint>,
    // children element end
}

/// Description: Each point in the trajectory.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct WayPoint {
    pub time: f64,
    pub pose: Pose,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = r#"<actor name="actor>
         <skin>
            <filename>moonwalk.dae</filename>
            <scale>1.0</scale>
         </skin>
         <pose>0 0 0 0 0 0</pose>
         <animation name="walking">
            <filename>walk.dae</filename>
            <scale>1</scale>
            <interpolate_x>true</interpolate_x>
         </animation>
         <script>
            <loop>true</loop>
            <delay_start>0</delay_start>
            <auto_start>true</auto_start>
            <trajectory id="0" type="walking">
               <waypoint>
                  <time>0</time>
                  <pose>0 1 0 0 0 0</pose>
               </waypoint>
               <waypoint>
                  <time>0.5</time>
                  <pose>0.1950 0.9807 0 0 0 -0.1963</pose>
               </waypoint>
               <waypoint>
                  <time>1</time>
                  <pose>0.3826 0.9238 0 0 0 -0.3926</pose>
               </waypoint>
               <waypoint>
                  <time>1.5</time>
                  <pose>0.5555 0.831 0 0 0 -0.589</pose>
               </waypoint>
            </trajectory>
         </script>
      </actor>"#;

    #[test]
    fn it_works() {
        let actor = Actor {
            name: "123".to_string(),
            skin: Some(Skin {
                filename: "hehe".to_string(),
                scale: Some(1.0),
            }),
            animations: vec![],
            script: None,
            pose: None,
            links: vec![],
            joints: vec![],
            plugins: vec![],
        };
        let xml = quick_xml::se::to_string(&actor).unwrap();
        // println!("{:#?}", xml);
        assert_eq!(
            "<Actor name=\"123\"><skin><filename>hehe</filename><scale>1</scale></skin></Actor>",
            xml
        );
    }
}
