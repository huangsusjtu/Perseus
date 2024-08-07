/// align to doc: http://sdformat.org/spec?ver=1.11&elem=sensor
/// finished
use serde::{Deserialize, Serialize};

use crate::sdf::util::{Plugin, Pose};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Sensor {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub r#type: String,

    // children element start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_on: Option<bool>, // default false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_rate: Option<f64>, // default 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualize: Option<bool>, // default false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>, // default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_metrics: Option<bool>, // default false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>, // default 0 0 0 0 0 0
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugin: Vec<Plugin>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_pressure: Option<sensor::AirPressure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_speed: Option<sensor::AirSpeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altimeter: Option<sensor::AltiMeter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<sensor::Camera>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<sensor::Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_torque: Option<sensor::ForceTorque>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps: Option<sensor::Gps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imu: Option<sensor::Imu>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lidar: Option<sensor::Lidar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic_camera: Option<sensor::LogicCamera>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnetometer: Option<sensor::MagnetoMeter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navsat: Option<sensor::Navsat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ray: Option<sensor::Ray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfidtag: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sonar: Option<sensor::Sonar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transceiver: Option<sensor::Transceiver>,
}

pub mod sensor {
    use serde::{Deserialize, Serialize};

    use crate::sdf::util::Pose;

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct AirPressure {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reference_altitude: Option<f64>, // default 0

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pressure: Option<Pressure>, // default 0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Pressure {
        pub noise: Noise,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Noise {
        #[serde(rename = "@type")]
        pub r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mean: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stddev: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bias_mean: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bias_stddev: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dynamic_bias_stddev: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dynamic_bias_correlation_time: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub precision: Option<f64>, // default 0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct AirSpeed {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pressure: Option<Pressure>, // default 0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct VerticalPosition {
        pub noise: Noise,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct VerticalVelocity {
        pub noise: Noise,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct AltiMeter {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub vertical_position: Option<VerticalPosition>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub vertical_velocity: Option<VerticalVelocity>, // default 0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Camera {
        #[serde(rename = "@name")]
        pub name: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub triggered: Option<bool>, // default false
        #[serde(skip_serializing_if = "Option::is_none")]
        pub camera_info_topic: Option<String>, // default
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger_topic: Option<String>, // default

        pub horizontal_fov: f64, // default 1.047
        pub image: camera::Image,
        pub clip: camera::Clip,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub save: Option<camera::Save>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub depth_camera: Option<camera::DepthCamera>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub segmentation_type: Option<String>, // default  semantic
        #[serde(skip_serializing_if = "Option::is_none")]
        pub box_type: Option<String>, // default  2d

        #[serde(skip_serializing_if = "Option::is_none")]
        pub noise: Option<camera::Noise>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub distortion: Option<camera::Distortion>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lens: Option<camera::Lens>,

        // Description: Visibility mask of a camera. When (camera's
        // visibility_mask & visual's visibility_flags) evaluates to
        // non-zero, the visual will be visible to the camera.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub visibility_mask: Option<u32>, // default 4294967295
        #[serde(skip_serializing_if = "Option::is_none")]
        pub optical_frame_id: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pose: Option<Pose>,
    }

    pub mod camera {
        use serde::{Deserialize, Serialize};

        use crate::sdf::util::Vector2;

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Noise {
            // The type of noise. Currently supported types are: "gaussian"
            // (draw additive noise values independently for each
            // pixel from a Gaussian distribution).
            // Default: gaussian
            pub r#type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mean: Option<f64>, // default 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stddev: Option<f64>, // default 0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Image {
            pub width: u32,  // default 320
            pub height: u32, // default 240
            // Description:
            // (L8|L16|R_FLOAT16|R_FLOAT32|R8G8B8|B8G8R8|BAYER_RGGB8|BAYER_BGGR8|BAYER_GBRG8|BAYER_GRBG8)
            #[serde(skip_serializing_if = "Option::is_none")]
            pub format: Option<String>, // Default: R8G8B8
            #[serde(skip_serializing_if = "Option::is_none")]
            pub anti_aliasing: Option<u32>, // Default: 4
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Clip {
            pub near: f64, // Default: 0.10000000000000001
            pub far: f64,  // Default: 100
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Save {
            #[serde(rename = "@enabled")]
            pub enabled: bool, // Default: false
            // Description: The path name which will hold the frame data. If
            // path name is relative, then directory is relative to
            // current working directory.
            pub path: String,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct DepthCamera {
            pub output: String, // Default: depths

            #[serde(skip_serializing_if = "Option::is_none")]
            pub clip: Option<Clip>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Distortion {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub k1: Option<f64>, // Default: 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub k2: Option<f64>, // Default: 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub k3: Option<f64>, // Default: 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub p1: Option<f64>, // Default: 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub p2: Option<f64>, // Default: 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub center: Option<Vector2<f64>>, // Default: 0.5 0.5
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Lens {
            // Description: Type of the lens mapping. Supported values are
            // gnomonical, stereographic, equidistant,
            // equisolid_angle, orthographic, custom. For gnomonical
            // (perspective) projection, it is recommended to specify a
            // horizontal_fov of less than or equal to 90°
            // Default: stereographic
            pub r#type: String,

            pub scale_to_hfov: bool, // default true
            #[serde(skip_serializing_if = "Option::is_none")]
            pub custom_function: Option<CustomFunction>,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub cutoff_angle: Option<f64>, // Default: 1.5707

            #[serde(skip_serializing_if = "Option::is_none")]
            pub env_texture_size: Option<u32>, // Default: 256

            #[serde(skip_serializing_if = "Option::is_none")]
            pub intrinsics: Option<Intrinsics>,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub projection: Option<Projection>, // Default: 256
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct CustomFunction {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub c1: Option<f64>, // Default: 1
            #[serde(skip_serializing_if = "Option::is_none")]
            pub c2: Option<f64>, // Default: 1
            #[serde(skip_serializing_if = "Option::is_none")]
            pub c3: Option<f64>, // Default: 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub f: Option<f64>, // Default: 1
            // Description: Possible values are 'sin', 'tan' and 'id'
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fun: Option<String>, // Default: tan
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Intrinsics {
            pub fx: f64, // Default: 277
            pub fy: f64, // Default: 277
            pub cx: f64, // Default: 160
            pub cy: f64, // Default: 120
            pub s: f64,  // Default: 0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Projection {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub p_fx: Option<f64>, // Default: 277
            #[serde(skip_serializing_if = "Option::is_none")]
            pub p_fy: Option<f64>, // Default: 277
            #[serde(skip_serializing_if = "Option::is_none")]
            pub p_cx: Option<f64>, // Default: 160
            #[serde(skip_serializing_if = "Option::is_none")]
            pub p_cy: Option<f64>, // Default: 120
            #[serde(skip_serializing_if = "Option::is_none")]
            pub tx: Option<f64>, // Default: 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ty: Option<f64>, // Default: 0
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Contact {
        pub collision: String,
        pub topic: String,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct ForceTorque {
        // Description: Frame in which to report the wrench values. Currently
        // supported frames are: "parent" report the wrench expressed
        // in the orientation of the parent link frame, "child" report
        // the wrench expressed in the orientation of the child link frame,
        // "sensor" report the wrench expressed in the orientation of
        // the joint sensor frame. Note that for each option the point
        // with respect to which the torque component of the wrench is
        // expressed is the joint origin.
        // Default: child
        #[serde(skip_serializing_if = "Option::is_none")]
        pub frame: Option<String>,
        // Description: Direction of the wrench measured by the sensor. The
        // supported options are: "parent_to_child" if the measured
        // wrench is the one applied by the parent link on the
        // child link, "child_to_parent" if the measured wrench is the one
        // applied by the child link on the parent link.
        // Default: child_to_parent
        #[serde(skip_serializing_if = "Option::is_none")]
        pub measure_direction: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub force: Option<force_torque::Force>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub torque: Option<force_torque::Torque>,
    }

    pub mod force_torque {
        use serde::{Deserialize, Serialize};

        use crate::sdf::sensor::sensor::Noise;

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Force {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub x: Option<Noise>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub y: Option<Noise>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub z: Option<Noise>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Torque {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub x: Option<Noise>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub y: Option<Noise>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub z: Option<Noise>,
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Gps {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub position_sensing: Option<gps::PositionSensing>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub velocity_sensing: Option<gps::VelocitySensing>,
    }

    pub mod gps {
        use serde::{Deserialize, Serialize};

        use crate::sdf::sensor::sensor::Noise;

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct PositionSensing {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub horizontal: Option<Horizontal>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vertical: Option<Vertical>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct VelocitySensing {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub horizontal: Option<Horizontal>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vertical: Option<Vertical>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Horizontal {
            pub noise: Noise,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Vertical {
            pub noise: Noise,
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Imu {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orientation_reference_frame: Option<imu::OrientationReferenceFrame>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub angular_velocity: Option<imu::AngularVelocity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub linear_acceleration: Option<imu::LinearAcceleration>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enable_orientation: Option<bool>, // default true
    }

    pub mod imu {
        use serde::{Deserialize, Serialize};

        use crate::sdf::sensor::sensor::Noise;
        use crate::sdf::util::Vector3;

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct OrientationReferenceFrame {
            // Description: This string represents special hardcoded use cases
            // that are commonly seen with typical robot IMU's: -
            // CUSTOM: use Euler angle custom_rpy orientation
            // specification. The orientation of the IMU's reference frame is
            // defined by adding the custom_rpy rotation to the
            // parent_frame. - NED: The IMU XYZ aligns with NED, where
            // NED orientation relative to Gazebo world is defined by the
            // SphericalCoordinates class. - ENU: The IMU XYZ
            // aligns with ENU, where ENU orientation relative to Gazebo
            // world is defined by the SphericalCoordinates class. - NWU: The
            // IMU XYZ aligns with NWU, where NWU orientation
            // relative to Gazebo world is defined by the
            // SphericalCoordinates class. - GRAV_UP: where direction of
            // gravity maps to IMU reference frame Z-axis with
            // Z-axis pointing in the opposite direction of gravity.
            // IMU reference frame X-axis direction is defined by grav_dir_x.
            // Note if grav_dir_x is parallel to gravity direction,
            // this configuration fails. Otherwise, IMU reference
            // frame X-axis is defined by projection of grav_dir_x onto a plane
            // normal to the gravity vector. IMU reference frame
            // Y-axis is a vector orthogonal to both X and Z
            // axis following the right hand rule. - GRAV_DOWN: where direction
            // of gravity maps to IMU reference frame Z-axis with
            // Z-axis pointing in the direction of gravity. IMU
            // reference frame X-axis direction is defined by grav_dir_x. Note
            // if grav_dir_x is parallel to gravity direction, this
            // configuration fails. Otherwise, IMU reference
            // frame X-axis is defined by projection of grav_dir_x onto a plane
            // normal to the gravity vector. IMU reference frame
            // Y-axis is a vector orthogonal to both X and Z
            // axis following the right hand rule.
            pub localization: String, // Default: CUSTOM

            #[serde(skip_serializing_if = "Option::is_none")]
            pub custom_rpy: Option<Vector3<f64>>, // Default: 0 0 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub grav_dir_x: Option<Vector3<f64>>, // Default: 1 0 0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct AngularVelocity {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub x: Option<Noise>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub y: Option<Noise>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub z: Option<Noise>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct LinearAcceleration {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub x: Option<Noise>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub y: Option<Noise>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub z: Option<Noise>,
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Lidar {
        pub scan: lidar::Scan,
        pub range: lidar::Range,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub noise: Option<lidar::Noise>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub visibility_mask: Option<u32>, // default 4294967295
    }

    pub mod lidar {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Scan {
            pub horizontal: Horizontal,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vertical: Option<Vertical>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Range {
            pub min: f64, // default 0
            pub max: f64, // default 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resolution: Option<f64>, // default 0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Noise {
            // The type of noise. Currently supported types are: "gaussian"
            // (draw additive noise values independently for each
            // pixel from a Gaussian distribution).
            // Default: gaussian
            pub r#type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mean: Option<f64>, // default 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stddev: Option<f64>, // default 0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Horizontal {
            // Description: The number of simulated lidar rays to generate per
            // complete laser sweep cycle.
            pub samples: u32, // default 640
            // Description: This number is multiplied by samples to determine
            // the number of range data points returned. If
            // resolution is not equal to one, range data is interpolated.
            pub resolution: f64, // default 1
            pub min_angle: f64,  // default 0
            pub max_angle: f64,  // default 0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Vertical {
            pub samples: u32, // default 1
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resolution: Option<f64>, // default 1
            pub min_angle: f64, // default 0
            pub max_angle: f64, // default 0
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct LogicCamera {
        pub near: f64,           // 0
        pub far: f64,            // 1
        pub aspect_ratio: f64,   // 1
        pub horizontal_fov: f64, // 1
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct MagnetoMeter {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub x: Option<Noise>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub y: Option<Noise>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub z: Option<Noise>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Navsat {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub position_sensing: Option<navsat::PositionSensing>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub velocity_sensing: Option<navsat::VelocitySensing>,
    }

    pub mod navsat {
        use serde::{Deserialize, Serialize};

        use crate::sdf::sensor::sensor::Noise;

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct PositionSensing {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub horizontal: Option<crate::sdf::sensor::sensor::gps::Horizontal>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vertical: Option<crate::sdf::sensor::sensor::gps::Vertical>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct VelocitySensing {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub horizontal: Option<crate::sdf::sensor::sensor::gps::Horizontal>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vertical: Option<crate::sdf::sensor::sensor::gps::Vertical>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Horizontal {
            pub noise: Noise,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Vertical {
            pub noise: Noise,
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Ray {
        pub scan: ray::Scan,
        pub range: ray::Range,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub noise: Option<ray::Noise>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub visibility_mask: Option<u32>, // default 4294967295
    }

    pub mod ray {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Scan {
            pub horizontal: crate::sdf::sensor::sensor::lidar::Horizontal,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vertical: Option<crate::sdf::sensor::sensor::lidar::Vertical>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Range {
            pub min: f64, // default 0
            pub max: f64, // default 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resolution: Option<f64>, // default 0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Noise {
            // The type of noise. Currently supported types are: "gaussian"
            // (draw additive noise values independently for each
            // pixel from a Gaussian distribution).
            // Default: gaussian
            pub r#type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mean: Option<f64>, // default 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stddev: Option<f64>, // default 0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Horizontal {
            // Description: The number of simulated lidar rays to generate per
            // complete laser sweep cycle.
            pub samples: u32, // default 640
            // Description: This number is multiplied by samples to determine
            // the number of range data points returned. If
            // resolution is not equal to one, range data is interpolated.
            pub resolution: f64, // default 1
            pub min_angle: f64,  // default 0
            pub max_angle: f64,  // default 0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Vertical {
            pub samples: u32, // default 1
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resolution: Option<f64>, // default 1
            pub min_angle: f64, // default 0
            pub max_angle: f64, // default 0
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Sonar {
        // Description: The sonar collision shape. Currently supported
        // geometries are: "cone" and "sphere".
        // Default: cone
        #[serde(skip_serializing_if = "Option::is_none")]
        pub geometry: Option<String>,

        pub min: f64, // default 0
        pub max: f64, // default 1

        #[serde(skip_serializing_if = "Option::is_none")]
        pub radius: Option<f64>, // default 0.5
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Transceiver {
        // Default: wireless
        #[serde(skip_serializing_if = "Option::is_none")]
        pub essid: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub frequency: Option<f64>, // Default: 2442
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_frequency: Option<f64>, // Default: 2412
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_frequency: Option<f64>, // Default: 2484

        pub gain: f64,  // 2.5
        pub power: f64, // 14.5

        #[serde(skip_serializing_if = "Option::is_none")]
        pub sensitivity: Option<f64>, // Default: -90
    }
}
