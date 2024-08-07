fn main() {
    println!("cargo:rerun-if-changed=src/api");
    protobuf_codegen::Codegen::new()
        .protoc()
        .includes(["src/proto"])
        .inputs(["src/proto/map.proto"])
        .out_dir("src/proto_gen")
        .run_from_script();
}
