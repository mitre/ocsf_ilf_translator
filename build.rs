/**
 * Copyright 2025 The MITRE Corporation

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */

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
