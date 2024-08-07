/// align to doc: http://sdformat.org/spec?ver=1.11&elem=physics
/// finished
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Physics {
    // attribute start
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@default", skip_serializing_if = "Option::is_none")]
    pub r#default: Option<bool>, // default false
    pub r#type: Option<String>, // default ode
    // attribute end

    // children element start
    pub max_step_size: f64,         // default 0.001
    pub real_time_factor: f64,      // default 1
    pub real_time_update_rate: f64, // default 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contacts: Option<i32>, // default 20

    // dynamic engine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dart: Option<Dart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simbody: Option<SimBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet: Option<Bullet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ode: Option<Ode>,
    // children element end
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Dart {
    pub solver: dart::Solver,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_detector: Option<String>,
    /* Specify collision detector
                                                * for DART to use. Can
                                                * be dart, fcl, bullet or ode. */
}

pub mod dart {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Solver {
        pub solver_type: String,
        /*  One of the following types: pgs,
                                         * dantzig. PGS stands for
                                         * Projected Gauss-Seidel. */
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct SimBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_step_size: Option<f64>, // default 0.0001,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>, // default 0.001,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_transient_velocity: Option<f64>, // default 0.01,
    // Description: Relationship among dissipation, coef. restitution, etc. d =
    // dissipation coefficient (1/velocity) vc = capture velocity (velocity
    // where e=e_max) vp = plastic velocity (smallest v where e=e_min) > vc
    // Assume real COR=1 when v=0. e_min = given minimum COR, at v >= vp
    // (a.k.a. plastic_coef_restitution) d = slope = (1-e_min)/vp OR, e_min
    // = 1 - d*vp e_max = maximum COR = 1-d*vc, reached at v=vc e = 0, v <= vc
    // = 1 - d*v, vc < v < vp = e_min, v >= vp dissipation factor =
    // d*min(v,vp) [compliant] cor = e [rigid] Combining rule e = 0,
    // e1==e2==0 = 2*e1*e2/(e1+e2), otherwise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<simbody::Contact>, // default 0.001,
}

pub mod simbody {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Contact {
        //  Default contact material stiffness (force/dist or torque/radian).
        // default 100000000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stiffness: Option<f64>,
        // dissipation coefficient to be used in compliant contact; if not
        // given it is (1-min_cor)/plastic_impact_velocity. default 100
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dissipation: Option<f64>,
        //  this is the COR to be used at high velocities for rigid impacts; if
        // not given it is 1
        // - dissipation*plastic_impact_velocity. default 0.5
        #[serde(skip_serializing_if = "Option::is_none")]
        pub plastic_coef_restitution: Option<f64>,
        // smallest impact velocity at which min COR is reached; set to zero if
        // you want the min COR always to be used. default 0.5
        #[serde(skip_serializing_if = "Option::is_none")]
        pub plastic_impact_velocity: Option<f64>,

        // static friction (mu_s) as described by this plot: http://gazebosim.org/wiki/File:Stribeck_friction.png default 0.9
        #[serde(skip_serializing_if = "Option::is_none")]
        pub static_frictio: Option<f64>,
        // dynamic friction (mu_d) as described by this plot: http://gazebosim.org/wiki/File:Stribeck_friction.png default 0.9
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dynamic_frictio: Option<f64>,

        // viscous friction (mu_v) with units of (1/velocity) as described by this plot: http://gazebosim.org/wiki/File:Stribeck_friction.png. default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub viscous_friction: Option<f64>,

        // for rigid impacts only, impact velocity at which COR is set to zero;
        // normally inherited from global default but can be overridden
        // here. Combining rule: use larger velocity, default 0.001
        #[serde(skip_serializing_if = "Option::is_none")]
        pub override_impact_capture_velocity: Option<f64>,

        // This is the largest slip velocity at which we'll consider
        // a transition to stiction. Normally inherited from a global
        // default setting. For a continuous friction model this is
        // the velocity at which the max static friction force is
        // reached. Combining rule: use larger velocity
        // default 0.001
        #[serde(skip_serializing_if = "Option::is_none")]
        pub override_stiction_transition_velocity: Option<f64>,
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Bullet {
    pub solver: bullet::Solver,
    pub constraints: bullet::Constraints,
}

pub mod bullet {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Solver {
        // One of the following types: sequential_impulse only.
        pub r#type: String,
        // The time duration which advances with each iteration of the dynamics
        // engine, this has to be no bigger than max_step_size under
        // physics block. If left unspecified, min_step_size defaults
        // to max_step_size. default 0.0001
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_step_size: Option<f64>,

        // Number of iterations for each step. A higher number produces greater
        // accuracy at a performance cost. default 50
        pub iters: usize,

        // Set the successive over-relaxation parameter.
        // default 1.3
        pub sor: f64,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Constraints {
        // Constraint force mixing parameter. See the ODE page for more
        // information. default 0
        pub cfm: f64,
        //  Error reduction parameter. See the ODE page for more information.
        // default 0.2
        pub erp: f64,

        // The depth of the surface layer around all geometry objects. Contacts
        // are allowed to sink into the surface layer up to the given
        // depth before coming to rest. The default value is zero.
        // Increasing this to some small value (e.g. 0.001) can help prevent
        // jittering problems due to contacts being repeatedly made and broken.
        // default 0.001
        pub contact_surface_layer: f64,

        // Similar to ODE's max_vel implementation.
        // See http://web.archive.org/web/20120430155635/http://bulletphysics.org/mediawiki-1.5.8/index.php/BtContactSolverInfo#Split_Impulse for more information.
        // default true,
        pub split_impulse: bool,

        // Similar to ODE's max_vel implementation.
        // See http://web.archive.org/web/20120430155635/http://bulletphysics.org/mediawiki-1.5.8/index.php/BtContactSolverInfo#Split_Impulse for more information.
        // default -0.01
        pub split_impulse_penetration_threshold: f64,
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Ode {
    pub solver: ode::Solver,
    pub constraints: ode::Constraints,
}

pub mod ode {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Solver {
        // One of the following types: world, quick
        pub r#type: String,
        // The time duration which advances with each iteration of the dynamics
        // engine, this has to be no bigger than max_step_size under
        // physics block. If left unspecified, min_step_size defaults
        // to max_step_size. Default: 0.0001
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_step_size: Option<f64>,

        // Number of threads to use for "islands" of disconnected models.
        // Default: 0
        pub island_threads: Option<usize>,

        // Number of iterations for each step. A higher number produces greater
        // accuracy at a performance cost. default 50
        pub iters: usize,

        // default 0
        pub precon_iters: Option<usize>,

        // Set the successive over-relaxation parameter.
        // default 1.3
        pub sor: f64,

        // Flag to use threading to speed up position correction computation.
        // Default: false
        pub thread_position_correction: Option<bool>,

        // Flag to enable dynamic rescaling of moment of inertia in constrained
        // directions. See gazebo pull request 1114 for the implementation of this feature. https://osrf-migration.github.io/gazebo-gh-pages/#!/osrf/gazebo/pull-request/1114
        // Default: false
        pub use_dynamic_moi_rescaling: bool,

        // Name of ODE friction model to use.
        // Valid values include: pyramid_model: (default) friction forces
        // limited in two directions in proportion to normal force.
        // box_model: friction forces limited to constant in two directions. cone_model: friction force magnitude limited in proportion to normal force. See gazebo pull request 1522 for the implementation of this feature. https://osrf-migration.github.io/gazebo-gh-pages/#!/osrf/gazebo/pull-request/1522 https://github.com/osrf/gazebo/commit/968dccafdfbfca09c9b3326f855612076fed7e6f
        // Default: pyramid_model
        pub friction_model: Option<String>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Constraints {
        // Constraint force mixing parameter. See the ODE page for more
        // information. default 0
        pub cfm: f64,
        //  Error reduction parameter. See the ODE page for more information.
        // default 0.2
        pub erp: f64,

        // The maximum correcting velocities allowed when resolving contacts.
        // Default: 100
        pub contact_max_correcting_vel: f64,

        // The depth of the surface layer around all geometry objects. Contacts
        // are allowed to sink into the surface layer up to the given
        // depth before coming to rest. The default value is zero.
        // Increasing this to some small value (e.g. 0.001) can help prevent
        // jittering problems due to contacts being repeatedly made and broken.
        // default 0.001
        pub contact_surface_layer: f64,
    }
}
