use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct PointF {
    x: f32,
    y: f32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let mut v = vec![];
    v.push(p.clone());
    v.push(p);

    let serialized = serde_json::to_string(&v).unwrap();
    println!("serialized: {}", serialized);

    let deserialized: Vec<Point> = serde_json::from_str(&serialized).unwrap();
    println!("Point deserialized: {:?}", deserialized);

    let deserialized: Vec<PointF> = serde_json::from_str(&serialized).unwrap();
    println!("PointF deserialized: {:?}", deserialized);

    // yml example
    parse_yml()
}

fn parse_yml() {
    let file =
        std::fs::File::open("config.yml").expect("config.yml not found!");
    let cfg: serde_yaml::Value =
        serde_yaml::from_reader(file).expect("could not read yaml");
    let a = get_value_vec(&cfg, "pack_files")
        .into_iter()
        .map(|v| {
            (
                get_value_string(&v, "file").replace("\n", ""),
                get_value_vec(&v, "flag")
                    .into_iter()
                    .map(|v| v.as_u64().unwrap() as u8)
                    .collect::<Vec<u8>>(),
            )
        })
        .collect::<Vec<(String, Vec<u8>)>>();
    dbg!(a);
}
fn get_value_vec(v: &serde_yaml::Value, label: &str) -> Vec<serde_yaml::Value> {
    v[label]
        .as_sequence()
        .unwrap_or_else(|| panic!("{} field not found", label))
        .to_vec()
}
fn get_value_string(v: &serde_yaml::Value, label: &str) -> String {
    serde_yaml::to_string(&v[label])
        .unwrap_or_else(|e| panic!("could not parse string from: {label}. {e}"))
}
