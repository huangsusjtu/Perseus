

pub use map::SDMap;
pub type MapRef = std::sync::Arc<SDMap>;
pub use element::*;
pub use common::util::LanePoint;
pub mod common;
mod element;
mod generator;
mod map;
pub mod proto_gen;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::common::util::LanePoint;
    use crate::element::{ConnectionInfo, JunctionInfo};
    use crate::element::{GeometryLine, StraightLine};
    use crate::element::{RoadInfo, RoadType};
    use crate::element::{SiteInfo, SiteType};
    use crate::map::SDMap;

    #[test]
    fn it_works0() {
        let p = std::env::current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join("libformat")
            .join("asset")
            .join("opendrive")
            .join("circular_road.xodr");
        let map = SDMap::load_map_file(p).unwrap();
        println!("{:#?}", map);
    }

    #[test]
    fn it_works1() {
        let mut map = SDMap::new();
        map.roads.insert(
            1,
            Arc::new(RoadInfo {
                id: 1,
                name: "r-1".to_string(),
                length: 0.0,
                r#type: RoadType::Major,
                center_line: crate::element::LineCurveInfo::new(
                    vec![GeometryLine::Straight(StraightLine {
                        hdg: 0.0,
                        length: 10.0,
                        s: 0.0,
                        x: 0.0,
                        y: 0.0,
                    })],
                ),
                left_lanes: vec![],
                right_lanes: vec![],
                link: vec![],
            }),
        );
        map.junctions.insert(
            1,
            Arc::new(JunctionInfo {
                id: 1,
                name: "j-1".to_string(),
                center: LanePoint::new(1.0, 1.0),
                polygon: vec![
                    LanePoint::new(1.0, 1.0),
                    LanePoint::new(1.0, 2.0),
                    LanePoint::new(2.0, 1.0),
                    LanePoint::new(2.0, 2.0),
                ],
                road_connections: vec![ConnectionInfo {
                    id: 1,
                    road_in: "r-1".to_string(),
                    road_out: "r-2".to_string(),
                }],
            }),
        );
        map.sites.insert(
            1,
            Arc::new(SiteInfo {
                id: 1,
                r#type: SiteType::CleanSite,
                name: "s-1".to_string(),
                position: LanePoint::new(20.0, 10.0),
            }),
        );

        map.store_map_file(std::env::current_dir().unwrap().join("1.xml"))
            .unwrap()
    }
}

// #[test]
// fn it_works_generate_sdmap() {
//     // junction center
//     //  121.41208524585619, 31.1781783163998
//     //  121.41354041248064, 31.178496049661465
//     //  121.41592759512667, 31.17907616288467
//     //  121.41722843395026, 31.175121641796242
//     //  121.41448881671677, 31.174558604527448
//     //  121.41285114695684, 31.17421001652996
//
//     let mut junction1 = JunctionInfo {
//         id: "j-1".to_string(),
//         boundary: vec![LanePoint::new(121.41208524585619, 31.1781783163998)],
//         road_ids: vec!["r-1".to_string(), "r-6".to_string()],
//         lane_ids_in: vec!["l-11".to_string(), "l-2".to_string()],
//         lane_ids_out: vec!["l-1".to_string(), "l-12".to_string()],
//     };
//     let mut junction2 = JunctionInfo {
//         id: "j-2".to_string(),
//         boundary: vec![LanePoint::new(121.41354041248064,
// 31.178496049661465)],         road_ids: vec!["r-1".to_string(),
// "r-2".to_string(), "r-7".to_string()],         lane_ids_in:
// vec!["l-1".to_string(), "l-4".to_string(), "l-13".to_string()],
//         lane_ids_out: vec!["l-2".to_string(), "l-3".to_string(),
// "l-14".to_string()],     };
//     let mut junction3 = JunctionInfo {
//         id: "j-3".to_string(),
//         boundary: vec![LanePoint::new(121.41592759512667,
// 31.17907616288467)],         road_ids: vec!["r-2".to_string(),
// "r-3".to_string()],         lane_ids_in: vec!["l-3".to_string(),
// "l-6".to_string()],         lane_ids_out: vec!["l-4".to_string()],
//     };
//     let mut junction4 = JunctionInfo {
//         id: "j-4".to_string(),
//         boundary: vec![LanePoint::new(121.41722843395026,
// 31.175121641796242)],         road_ids: vec!["r-3".to_string(),
// "r-4".to_string()],         lane_ids_in: vec!["l-8".to_string()],
//         lane_ids_out: vec!["l-6".to_string(), "l-7".to_string()],
//     };
//     let mut junction5 = JunctionInfo {
//         id: "j-5".to_string(),
//         boundary: vec![LanePoint::new(121.41448881671677,
// 31.174558604527448)],         road_ids: vec!["r-4".to_string(),
// "r-5".to_string(), "r-7".to_string()],         lane_ids_in:
// vec!["l-14".to_string(), "l-7".to_string(), "l-10".to_string()],
//         lane_ids_out: vec!["l-13".to_string(), "l-8".to_string(),
// "l-9".to_string()],     };
//     let mut junction6 = JunctionInfo {
//         id: "j-6".to_string(),
//         boundary: vec![LanePoint::new(121.41285114695684,
// 31.17421001652996)],         road_ids: vec!["r-5".to_string(),
// "r-6".to_string()],         lane_ids_in: vec!["l-9".to_string(),
// "l-12".to_string()],         lane_ids_out: vec!["l-10".to_string(),
// "l-11".to_string()],     };
//
//     let lane1 = LaneInfo::new(
//         1,
//         vec![
//             (121.41202199999991, 31.17808985999365),
//             (121.41208199999993, 31.17811185999615),
//             (121.41241099999992, 31.178187860004797),
//             (121.41262999999992, 31.178241860010953),
//             (121.41292799999992, 31.178316860019486),
//             (121.41303799999991, 31.178338860021984),
//             (121.41352499999994, 31.17846886003678),
//         ],
//         None,
//         Some("j-1".to_string()),
//         Some("j-2".to_string()),
//         Some("r-1".to_string()),
//     );
//     let lane2 = LaneInfo::new(
//         2,
//         vec![
//             (121.41352499999994, 31.17846886003678),
//             (121.41303799999991, 31.178338860021984),
//             (121.41292799999992, 31.178316860019486),
//             (121.41262999999992, 31.178241860010953),
//             (121.41241099999992, 31.178187860004797),
//             (121.41208199999993, 31.17811185999615),
//             (121.41202199999991, 31.17808985999365),
//         ],
//         None,
//         Some("j-2".to_string()),
//         Some("j-1".to_string()),
//         Some("r-1".to_string()),
//     );
//     let lane3 = LaneInfo::new(
//         3,
//         vec![
//             (121.41353499999992, 31.17846886003678),
//             (121.41364499999993, 31.17849086003929),
//             (121.41437099999992, 31.178674860060244),
//             (121.41484899999993, 31.178783860072663),
//             (121.41527699999993, 31.17888186008385),
//             (121.41543599999993, 31.178914860087605),
//             (121.41598299999991, 31.179035860101408),
//             (121.41598299999991, 31.179035860101408),
//             (121.41596299999992, 31.179124860111568),
//             (121.41596299999992, 31.179124860111568),
//             (121.41591399999993, 31.179113860110306),
//         ],
//         None,
//         Some("j-2".to_string()),
//         Some("j-3".to_string()),
//         Some("r-2".to_string()),
//     );
//     let lane4 = LaneInfo::new(
//         4,
//         vec![
//             (121.41591399999993, 31.179113860110306),
//             (121.41596299999992, 31.179124860111568),
//             (121.41596299999992, 31.179124860111568),
//             (121.41598299999991, 31.179035860101408),
//             (121.41598299999991, 31.179035860101408),
//             (121.41543599999993, 31.178914860087605),
//             (121.41527699999993, 31.17888186008385),
//             (121.41484899999993, 31.178783860072663),
//             (121.41437099999992, 31.178674860060244),
//             (121.41364499999993, 31.17849086003929),
//             (121.41353499999992, 31.17846886003678),
//         ],
//         None,
//         Some("j-3".to_string()),
//         Some("j-2".to_string()),
//         Some("r-2".to_string()),
//     );
//
//     let lane6 = LaneInfo::new(
//         6,
//         vec![
//             (121.41720699999993, 31.17510885965626),
//             (121.41718699999993, 31.1751988596664),
//             (121.41700799999992, 31.175754859729096),
//             (121.41684899999991, 31.17626185978637),
//             (121.41667999999993, 31.176788859846003),
//             (121.41663999999992, 31.176907859859483),
//             (121.41655999999993, 31.17714685988657),
//             (121.41651099999993, 31.17729585990347),
//             (121.41639099999993, 31.17766285994512),
//             (121.41628199999992, 31.178010859984674),
//             (121.41625199999993, 31.178110859996046),
//             (121.41615199999993, 31.17839886002882),
//             (121.41598299999991, 31.179035860101408),
//             (121.41596299999992, 31.179124860111568),
//             (121.41596299999992, 31.179124860111568),
//             (121.41591399999993, 31.179113860110306),
//         ],
//         None,
//         Some("j-4".to_string()),
//         Some("j-3".to_string()),
//         Some("r-3".to_string()),
//     );
//     let lane7 = LaneInfo::new(
//         7,
//         vec![
//             (121.41720699999993, 31.17510885965626),
//             (121.41718699999993, 31.1751988596664),
//             (121.41718699999993, 31.1751988596664),
//             (121.41630199999993, 31.175001859644212),
//             (121.41622199999992, 31.17497985964173),
//             (121.41594299999993, 31.174924859635542),
//             (121.41557499999993, 31.17483785962575),
//             (121.41517699999993, 31.174759859616973),
//             (121.41447999999991, 31.17460685959977),
//             (121.41447999999991, 31.17460685959977),
//             (121.41448999999993, 31.174556859594148),
//         ],
//         None,
//         Some("j-4".to_string()),
//         Some("j-5".to_string()),
//         Some("r-4".to_string()),
//     );
//     let lane8 = LaneInfo::new(
//         8,
//         vec![
//             (121.41448999999993, 31.174556859594148),
//             (121.41447999999991, 31.17460685959977),
//             (121.41447999999991, 31.17460685959977),
//             (121.41517699999993, 31.174759859616973),
//             (121.41557499999993, 31.17483785962575),
//             (121.41594299999993, 31.174924859635542),
//             (121.41622199999992, 31.17497985964173),
//             (121.41630199999993, 31.175001859644212),
//             (121.41718699999993, 31.1751988596664),
//             (121.41718699999993, 31.1751988596664),
//             (121.41720699999993, 31.17510885965626),
//         ],
//         None,
//         Some("j-5".to_string()),
//         Some("j-4".to_string()),
//         Some("r-4".to_string()),
//     );
//     let lane9 = LaneInfo::new(
//         9,
//         vec![
//             (121.41439099999991, 31.174554859593922),
//             (121.41447999999991, 31.17460685959977),
//             (121.41431099999993, 31.17461385960054),
//             (121.41412199999994, 31.174569859595593),
//             (121.41378399999994, 31.17450385958819),
//             (121.41361499999992, 31.174459859583234),
//             (121.41339599999992, 31.17439585957605),
//             (121.41313699999994, 31.17432085956761),
//             (121.41293799999993, 31.174277859562785),
//         ],
//         None,
//         Some("j-5".to_string()),
//         Some("j-6".to_string()),
//         Some("r-5".to_string()),
//     );
//     let lane10 = LaneInfo::new(
//         10,
//         vec![
//             (121.41293799999993, 31.174277859562785),
//             (121.41313699999994, 31.17432085956761),
//             (121.41339599999992, 31.17439585957605),
//             (121.41361499999992, 31.174459859583234),
//             (121.41378399999994, 31.17450385958819),
//             (121.41412199999994, 31.174569859595593),
//             (121.41431099999993, 31.17461385960054),
//             (121.41447999999991, 31.17460685959977),
//             (121.41439099999991, 31.174554859593922),
//         ],
//         None,
//         Some("j-6".to_string()),
//         Some("j-5".to_string()),
//         Some("r-5".to_string()),
//     );
//     let lane11 = LaneInfo::new(
//         11,
//         vec![
//             (121.41283899999993, 31.174225859556948),
//             (121.41260999999993, 31.17555085970608),
//             (121.41242999999993, 31.176387859800627),
//             (121.41225099999993, 31.177274859901093),
//             (121.41208199999993, 31.17811185999615),
//             (121.41207199999991, 31.17818086000401),
//             (121.41206199999993, 31.17822086000856),
//         ],
//         None,
//         Some("j-6".to_string()),
//         Some("j-1".to_string()),
//         Some("r-6".to_string()),
//     );
//     let lane12 = LaneInfo::new(
//         12,
//         vec![
//             (121.41206199999993, 31.17822086000856),
//             (121.41207199999991, 31.17818086000401),
//             (121.41208199999993, 31.17811185999615),
//             (121.41225099999993, 31.177274859901093),
//             (121.41242999999993, 31.176387859800627),
//             (121.41260999999993, 31.17555085970608),
//             (121.41283899999993, 31.174225859556948),
//         ],
//         None,
//         Some("j-1".to_string()),
//         Some("j-6".to_string()),
//         Some("r-6".to_string()),
//     );
//     let lane13 = LaneInfo::new(
//         13,
//         vec![
//             (121.41448999999993, 31.174556859594148),
//             (121.41447999999991, 31.17460685959977),
//             (121.41441099999993, 31.174874859629913),
//             (121.41434099999991, 31.175193859665836),
//             (121.41426199999992, 31.175502859700664),
//             (121.41422199999992, 31.17569185972199),
//             (121.41410199999993, 31.17615985977484),
//             (121.41399299999993, 31.176606859825405),
//             (121.41390299999992, 31.176985859868317),
//             (121.41377399999993, 31.17761285993945),
//             (121.41364499999993, 31.17817086000286),
//             (121.41352499999994, 31.17846886003678),
//         ],
//         None,
//         Some("j-5".to_string()),
//         Some("j-2".to_string()),
//         Some("r-7".to_string()),
//     );
//     let lane14 = LaneInfo::new(
//         14,
//         vec![
//             (121.41352499999994, 31.17846886003678),
//             (121.41364499999993, 31.17817086000286),
//             (121.41377399999993, 31.17761285993945),
//             (121.41390299999992, 31.176985859868317),
//             (121.41399299999993, 31.176606859825405),
//             (121.41410199999993, 31.17615985977484),
//             (121.41422199999992, 31.17569185972199),
//             (121.41426199999992, 31.175502859700664),
//             (121.41434099999991, 31.175193859665836),
//             (121.41441099999993, 31.174874859629913),
//             (121.41447999999991, 31.17460685959977),
//             (121.41448999999993, 31.174556859594148),
//         ],
//         None,
//         Some("j-2".to_string()),
//         Some("j-5".to_string()),
//         Some("r-7".to_string()),
//     );
//
//     // 徐汇区公司园区附近一个小圈
//     let mut map = SDMap::new();
//     map.lane_infos.insert(lane1.id.clone(), Arc::new(lane1));
//     map.lane_infos.insert(lane2.id.clone(), Arc::new(lane2));
//     map.lane_infos.insert(lane3.id.clone(), Arc::new(lane3));
//     map.lane_infos.insert(lane4.id.clone(), Arc::new(lane4));
//     map.lane_infos.insert(lane6.id.clone(), Arc::new(lane6));
//     map.lane_infos.insert(lane7.id.clone(), Arc::new(lane7));
//     map.lane_infos.insert(lane8.id.clone(), Arc::new(lane8));
//     map.lane_infos.insert(lane9.id.clone(), Arc::new(lane9));
//     map.lane_infos.insert(lane10.id.clone(), Arc::new(lane10));
//     map.lane_infos.insert(lane11.id.clone(), Arc::new(lane11));
//     map.lane_infos.insert(lane12.id.clone(), Arc::new(lane12));
//     map.lane_infos.insert(lane13.id.clone(), Arc::new(lane13));
//     map.lane_infos.insert(lane14.id.clone(), Arc::new(lane14));
//     map.junctions.insert(junction1.id.clone(), Arc::new(junction1));
//     map.junctions.insert(junction2.id.clone(), Arc::new(junction2));
//     map.junctions.insert(junction3.id.clone(), Arc::new(junction3));
//     map.junctions.insert(junction4.id.clone(), Arc::new(junction4));
//     map.junctions.insert(junction5.id.clone(), Arc::new(junction5));
//     map.junctions.insert(junction6.id.clone(), Arc::new(junction6));
//     map.active();
//
//     map.store_map_file("/home/huangsu/text.bin", true).unwrap();
//     map.store_map_file("/home/huangsu/text.json", false).unwrap();
// }
//
// #[test]
// fn it_works_load_sdmap() {
//
//     println!("hehe");
// }
