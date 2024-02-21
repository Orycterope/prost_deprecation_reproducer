use std::path::PathBuf;

fn main() {
    let workspace_dir = PathBuf::from(".");
    let proto_path = workspace_dir.join("items.proto");
    let dst_path = workspace_dir.join("app").join("src");

    println!("proto_path is {:?}", proto_path);
    prost_build::Config::new()
        .out_dir(dst_path)
        .compile_protos(&[&proto_path], &[workspace_dir])
        .unwrap()
}
