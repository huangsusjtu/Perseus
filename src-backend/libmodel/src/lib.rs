pub mod robot;
pub mod sanitation;

#[cfg(test)]
mod tests {
    use crate::sanitation::environment::Environment;
    use crate::sanitation::garbage::GarbageConfig;
    use crate::sanitation::header::Header;
    use crate::sanitation::sanitationman::Sanitation;
    use crate::sanitation::scenario::{
        SanitationGroup, SanitationTaskGroup, Scenario, VehicleGroup,
        VehicleTaskGroup,
    };
    use crate::sanitation::task::{GlobalPlannerArea, GlobalPlannerPath};
    use crate::sanitation::traffic_flow::TrafficFlowConfig;
    use crate::sanitation::unparse;
    use crate::sanitation::vehicle::Vehicle;

    #[test]
    fn it_works() {
        let s = Scenario {
            header: Header::default(),
            environment: Environment::default(),
            vehicles: VehicleGroup {
                vehicles: vec![Vehicle::default(), Vehicle::default()],
            },
            sanitation_man: SanitationGroup {
                sanitation: vec![Sanitation::default(), Sanitation::default()],
            },

            global_vehicle_task: VehicleTaskGroup {
                task: vec![GlobalPlannerPath::default()],
            },

            global_man_task: SanitationTaskGroup {
                task: vec![GlobalPlannerArea::default()],
            },

            vehicle_to_task_mapper: vec![
                ("3".to_string(), "3".to_string()).into(),
                ("4".to_string(), "4".to_string()).into(),
            ]
            .into(),

            sanitation_to_task_mapper: vec![
                ("3".to_string(), "3".to_string()).into(),
                ("4".to_string(), "4".to_string()).into(),
            ]
            .into(),

            driver_to_vehicle_mapper: vec![
                ("3".to_string(), "3".to_string()).into(),
                ("4".to_string(), "4".to_string()).into(),
            ]
            .into(),
            traffic_flow_config: TrafficFlowConfig {},
            garbage_config: GarbageConfig {},
        };

        let c = unparse(&s).unwrap();
        println!("{:#?}", &s);
        println!("{}", &c);
        println!(
            "{:?}",
            std::env::current_dir().unwrap().join("asset").join("sanitation")
        );
        println!(
            "{:#?}",
            std::fs::write(
                std::env::current_dir()
                    .unwrap()
                    .join("asset")
                    .join("sanitation")
                    .join("1.xml"),
                &c,
            )
        );
    }
}
