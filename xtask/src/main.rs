use std::{env, fs, path::PathBuf};

mod gen;

fn main() {
    match env::args().nth(1) {
        Some(cmd) => match cmd.as_str() {
            "gen" => gen(),
            _ => print_help(),
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!(r#"cargo xtask gen"#)
}

fn gen() {
    let proto_root = PathBuf::from("xtask/proto/googleapis");
    let protos = gen::find_proto(proto_root.clone());

    // let gates = gen::feature_gates(&protos);
    // println!("{}", gates);

    let out_dir = PathBuf::from("googapis/genproto");
    let _ = fs::remove_dir_all(out_dir.as_path());
    let _ = fs::create_dir(out_dir.as_path());
    let includes = [proto_root];

    tonic_build::configure()
        .build_server(false)
        .format(false)
        .out_dir(out_dir.clone())
        .compile(&gen::proto_path(&protos), &includes)
        .unwrap();
    tonic_build::fmt(out_dir.to_str().unwrap());

    let broken_features = fs::read_to_string("googapis/broken-features.txt").unwrap();
    let broken_features = broken_features
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let cargo_toml_path = PathBuf::from("googapis/Cargo.toml");
    let mut cargo_toml = fs::read_to_string(&cargo_toml_path).unwrap();
    const FEATURES_MARKER: &str = "[features]\n";
    let features_start = cargo_toml.find(FEATURES_MARKER).unwrap();
    let gates = gen::feature_gates(&protos, &broken_features);
    cargo_toml.replace_range(
        features_start + FEATURES_MARKER.len()..,
        &format!("default = []\n{}\n", gates),
    );
    fs::write(&cargo_toml_path, cargo_toml).unwrap();

    let build_all_features_path = PathBuf::from("googapis/build-all-features.sh");
    let mut build_all_features = fs::read_to_string(&build_all_features_path).unwrap();
    const START_MARKER: &str = "# START FEATURES";
    const END_MARKER: &str = "# END FEATURES";
    let features_start = build_all_features.find(START_MARKER).unwrap();
    let features_end = build_all_features.find(END_MARKER).unwrap();
    let names = gen::feature_names(&protos, &broken_features);
    build_all_features.replace_range(
        features_start + START_MARKER.len()..features_end,
        &format!("\nfeatures=(\n  default\n{}\n)\n", names),
    );
    fs::write(&build_all_features_path, build_all_features).unwrap();

    let mut out_path = PathBuf::from("googapis/src/googapis.rs");
    let root = gen::from_protos(protos);
    fs::write(out_path.clone(), root.gen_code()).unwrap();

    out_path.pop();
    tonic_build::fmt(out_path.to_str().unwrap());
}
