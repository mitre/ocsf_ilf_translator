use protobuf_codegen::{Customize, CustomizeCallback};

struct GenILFSerialization;

impl CustomizeCallback for GenILFSerialization {
    fn message(
        &self,
        _message: &protobuf::reflect::MessageDescriptor,
    ) -> protobuf_codegen::Customize {
        Customize::default().before("#[derive(ilf_ocsf_macro::SerializeILF)]")
    }
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    // Use this in build.rs
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .include("src/protos")
        // Inputs must reside in some of include paths.
        .input("src/protos/ocsf.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        // Add Serde serilization
        .customize_callback(GenILFSerialization)
        // Run script
        .run_from_script();
}
