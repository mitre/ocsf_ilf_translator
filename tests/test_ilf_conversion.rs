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

#[test]
fn parse_ocsf_account_change_1() {
    let file_path = "ocsf_json_messages/1_1_0_OCSF_account_change.json";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_OCSF_account_change.json");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_eks() {
    let file_path = "ocsf_json_messages/ocsf_eks.json";
    let file_contents = include_str!("ocsf_json_messages/ocsf_eks.json");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_authentication_0() {
    let file_path = "ocsf_json_messages/1_0_0_OCSF_authentication.json";
    let file_contents = include_str!("ocsf_json_messages/1_0_0_OCSF_authentication.json");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_authentication_1() {
    let file_path = "ocsf_json_messages/1_1_0_OCSF_authentication.json";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_OCSF_authentication.json");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_api_activity_0() {
    let file_path = "ocsf_json_messages/1_0_0_OCSF_api_activity.json";
    let file_contents = include_str!("ocsf_json_messages/1_0_0_OCSF_api_activity.json");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_api_activity_1() {
    let file_path = "ocsf_json_messages/1_1_0_OCSF_api_activity.json";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_OCSF_api_activity.json");
    parse_and_translate_example(file_path, file_contents);
}

#[test]
fn parse_1_1_0_generic_api_activity() {
    let file_path = "ocsf_json_messages/1_1_0_generic_api_activity.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_generic_api_activity.ocsf");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_account_change() {
    let file_path = "ocsf_json_messages/1_1_0_account_change.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_account_change.ocsf");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_authentication() {
    let file_path = "ocsf_json_messages/1_1_0_authentication.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_authentication.ocsf");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_vpcflow() {
    let file_path = "ocsf_json_messages/1_1_0_vpcflow.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_vpcflow.ocsf");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_vpcflowlog() {
    let file_path = "ocsf_json_messages/1_1_0_vpcflowlog.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_vpcflowlog.ocsf");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_route53() {
    let file_path = "ocsf_json_messages/1_1_0_route53.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_route53.ocsf");
    parse_and_translate_example(file_path, file_contents);
}

#[test]
fn parse_sechub_pci() {
    let file_path = "ocsf_json_messages/sechub-pci.ocsf";
    let file_contents = include_str!("ocsf_json_messages/sechub-pci.ocsf");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_inspector() {
    let file_path = "ocsf_json_messages/inspector.ocsf";
    let file_contents = include_str!("ocsf_json_messages/inspector.ocsf");
    parse_and_translate_example(file_path, file_contents);
}
#[test]
fn parse_sechub_guardduty() {
    let file_path: &str = "ocsf_json_messages/sechub-guardduty.ocsf";
    let file_contents = include_str!("ocsf_json_messages/sechub-guardduty.ocsf");
    parse_and_translate_example(file_path, file_contents);
}

fn parse_and_translate_example(file_path: &str, file_contents: &str) {
    println!("Parsing file at {}", file_path);

    let _ilf = OCSFEvent::try_from(file_contents)
        .expect("Test file should parse")
        .to_ilf(true, true);
}
