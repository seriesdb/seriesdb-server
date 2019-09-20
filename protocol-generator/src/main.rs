use prost_build;

fn main() {
    build_by_prost();
}

fn build_by_prost() {
    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir("../src/protocol");
    prost_build.compile_protos(
        &["proto/seriesdb_protocol.proto"],
        &["proto/"]
    ).unwrap();
}