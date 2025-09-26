fn main() {
    let proto_dir = std::path::PathBuf::from("pb");
    let protos: Vec<_> = std::fs::read_dir(&proto_dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) == Some("proto") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    // println!("find protos: {:?}", protos);

    prost_build::Config::new()
        // 为所有类型添加 serde 支持
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        // 为枚举类型添加自定义序列化特性
        .type_attribute(".", "#[serde(rename_all = \"snake_case\")]")
        .out_dir("src/pb")
        .compile_protos(&protos, &[proto_dir])
        .unwrap();
}
