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