use serde::{Deserialize, Serialize};

pub mod core {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Validity {
        #[serde(rename = "@fromLane")]
        pub from_lane: i32,
        #[serde(rename = "@toLane")]
        pub to_lane: i32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub enum EUnit {
        EUnitDistance(EUnitDistance),
        EUnitSpeed(EUnitSpeed),
        EUnitMass(EUnitMass),
        EUnitSlope(EUnitSlope),
    }

    impl Default for EUnit {
        fn default() -> Self {
            EUnit::EUnitDistance(EUnitDistance::Ft)
        }
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EUnitDistance {
        #[default]
        #[serde(rename = "ft")]
        Ft,
        #[serde(rename = "km")]
        KM,
        #[serde(rename = "m")]
        M,
        #[serde(rename = "mile")]
        Mile,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EUnitSlope {
        #[default]
        #[serde(rename = "%")]
        Default,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EDataQualityRawDataSource {
        #[default]
        #[serde(rename = "cadaster")]
        Cadaster,
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "sensor")]
        Sensor,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EUnitSpeed {
        #[default]
        #[serde(rename = "km/h")]
        KmH,
        #[serde(rename = "m/s")]
        MS,
        #[serde(rename = "mph")]
        Mph,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EDataQualityRawDataPostProcessing {
        #[default]
        #[serde(rename = "cleaned")]
        Cleaned,
        #[serde(rename = "fused")]
        Fused,
        #[serde(rename = "processed")]
        Processed,
        #[serde(rename = "raw")]
        Raw,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EUnitMass {
        #[default]
        #[serde(rename = "kg")]
        KG,
        #[serde(rename = "t")]
        T,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum TYesNo {
        #[default]
        #[serde(rename = "yes")]
        Yes,
        #[serde(rename = "no")]
        No,
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Semantics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<Speed>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Speed {
    #[serde(rename = "@max")]
    pub max: f64, // >=0
    // km/h
    // m/s
    // mph
    #[serde(rename = "@unit")]
    pub unit: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Priority {
    #[serde(rename = "@high")]
    pub high: String,
    #[serde(rename = "@low")]
    pub low: String,
}

pub mod planview {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct PlanView {
        pub geometry: Vec<Geometry>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Geometry {
        #[serde(rename = "@hdg")]
        pub hdg: f32,
        #[serde(rename = "@length")]
        pub length: f32,
        #[serde(rename = "@s")]
        pub s: f32,
        #[serde(rename = "@x")]
        pub x: f64,
        #[serde(rename = "@y")]
        pub y: f64,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub line: Option<Line>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub spiral: Option<Spiral>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub arc: Option<Arc>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub poly3: Option<Poly3>,
        #[serde(rename = "paramPoly3", skip_serializing_if = "Option::is_none")]
        pub param_poly3: Option<ParamPoly3>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Line;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Spiral {
        #[serde(rename = "@curvStart")]
        pub curve_start: f32,
        #[serde(rename = "@curvEnd")]
        pub curve_end: f32,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Arc {
        #[serde(rename = "@curvature")]
        pub curvature: f32,
    }

    // deprecated,  no need impl
    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Poly3;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct ParamPoly3 {
        #[serde(rename = "@aU")]
        pub au: f32,
        #[serde(rename = "@aV")]
        pub av: f32,
        #[serde(rename = "@bU")]
        pub bu: f32,
        #[serde(rename = "@bV")]
        pub bv: f32,
        #[serde(rename = "@cU")]
        pub cu: f32,
        #[serde(rename = "@cV")]
        pub cv: f32,
        #[serde(rename = "@dU")]
        pub du: f32,
        #[serde(rename = "@dV")]
        pub dv: f32,
        #[serde(rename = "@pRange")]
        pub p_range: String, // arcLength  or normalized
    }
}
