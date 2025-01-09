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

use ilf_ocsf_protobuf::ocsf_types::OCSFEvent;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct OCSFConverter {
    #[structopt(long, short, parse(from_os_str))]
    ocsf_file_path: PathBuf,
}

fn main() {
    let OCSFConverter { ocsf_file_path } = OCSFConverter::from_args();

    let file_contents = match fs::read_to_string(ocsf_file_path.clone()) {
        Ok(c) => c,
        Err(e) => panic!("Could not read file {}. Error: {}", ocsf_file_path.display(), e),
    };
    let ocsf_file = OCSFEvent::try_from(file_contents.as_str()).expect("File should parse");
    let ilf = ocsf_file.to_ilf(true, true).unwrap();

    println!("result:\n{}", ilf)
}