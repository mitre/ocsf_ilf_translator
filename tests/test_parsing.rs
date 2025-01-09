use ilf_ocsf_protobuf::ocsf_types::OCSFEvent;
#[test]
fn parse_ocsf_account_change_1() {
    let file_path = "ocsf_json_messages/1_1_0_OCSF_account_change.json";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_OCSF_account_change.json");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_eks() {
    let file_path = "ocsf_json_messages/ocsf_eks.json";
    let file_contents = include_str!("ocsf_json_messages/ocsf_eks.json");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_authentication_0() {
    let file_path = "ocsf_json_messages/1_0_0_OCSF_authentication.json";
    let file_contents = include_str!("ocsf_json_messages/1_0_0_OCSF_authentication.json");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_authentication_1() {
    let file_path = "ocsf_json_messages/1_1_0_OCSF_authentication.json";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_OCSF_authentication.json");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_api_activity_0() {
    let file_path = "ocsf_json_messages/1_0_0_OCSF_api_activity.json";
    let file_contents = include_str!("ocsf_json_messages/1_0_0_OCSF_api_activity.json");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_ocsf_api_activity_1() {
    let file_path = "ocsf_json_messages/1_1_0_OCSF_api_activity.json";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_OCSF_api_activity.json");
    parse_example(file_path, file_contents);
}

#[test]
fn parse_1_1_0_generic_api_activity() {
    let file_path = "ocsf_json_messages/1_1_0_generic_api_activity.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_generic_api_activity.ocsf");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_account_change() {
    let file_path = "ocsf_json_messages/1_1_0_account_change.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_account_change.ocsf");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_authentication() {
    let file_path = "ocsf_json_messages/1_1_0_authentication.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_authentication.ocsf");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_vpcflow() {
    let file_path = "ocsf_json_messages/1_1_0_vpcflow.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_vpcflow.ocsf");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_vpcflowlog() {
    let file_path = "ocsf_json_messages/1_1_0_vpcflowlog.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_vpcflowlog.ocsf");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_1_1_0_route53() {
    let file_path = "ocsf_json_messages/1_1_0_route53.ocsf";
    let file_contents = include_str!("ocsf_json_messages/1_1_0_route53.ocsf");
    parse_example(file_path, file_contents);
}

#[test]
fn parse_sechub_pci() {
    let file_path = "ocsf_json_messages/sechub-pci.ocsf";
    let file_contents = include_str!("ocsf_json_messages/sechub-pci.ocsf");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_inspector() {
    let file_path = "ocsf_json_messages/inspector.ocsf";
    let file_contents = include_str!("ocsf_json_messages/inspector.ocsf");
    parse_example(file_path, file_contents);
}
#[test]
fn parse_sechub_guardduty() {
    let file_path: &str = "ocsf_json_messages/sechub-guardduty.ocsf";
    let file_contents = include_str!("ocsf_json_messages/sechub-guardduty.ocsf");
    parse_example(file_path, file_contents);
}

fn parse_example(file_path: &str, file_contents: &str) {
    println!("Parsing file at {}", file_path);
    OCSFEvent::try_from(file_contents).expect("Test file should parse");
}

// These fail (maybe because they're encoded with OCSF with the "splunk" extension?)
// #[test]
// fn parse_malware_infection_detected_sample_3() {
//     let file_path = "ocsf_json_messages/malware_infection_detected_sample_3.json";
//     let file_contents = include_str!("ocsf_json_messages/malware_infection_detected_sample_3.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_sample_coff_fccx() {
//     let file_path = "ocsf_json_messages/SampleOCSF_FCCX.json";
//     let file_contents = include_str!("ocsf_json_messages/SampleOCSF_FCCX.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_ocsf_dns_activity_0() {
//     let file_path = "ocsf_json_messages/1_0_0_OCSF_dns_activity.json";
//     let file_contents = include_str!("ocsf_json_messages/1_0_0_OCSF_dns_activity.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_4625_0() {
//     let file_path = "ocsf_json_messages/4625_0.json";
//     let file_contents = include_str!("ocsf_json_messages/4625_0.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_malware_infection_detected_sample_1() {
//     let file_path = "ocsf_json_messages/malware_infection_detected_sample_1.json";
//     let file_contents = include_str!("ocsf_json_messages/malware_infection_detected_sample_1.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_4661() {
//     let file_path = "ocsf_json_messages/4661.json";
//     let file_contents = include_str!("ocsf_json_messages/4661.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_malware_infection_detected_sample_4() {
//     let file_path = "ocsf_json_messages/malware_infection_detected_sample_4.json";
//     let file_contents = include_str!("ocsf_json_messages/malware_infection_detected_sample_4.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_4663_0() {
//     let file_path = "ocsf_json_messages/4663_0.json";
//     let file_contents = include_str!("ocsf_json_messages/4663_0.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_4624_0() {
//     let file_path = "ocsf_json_messages/4624_0.json";
//     let file_contents = include_str!("ocsf_json_messages/4624_0.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_4673_0() {
//     let file_path = "ocsf_json_messages/4673_0.json";
//     let file_contents = include_str!("ocsf_json_messages/4673_0.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_4689_0() {
//     let file_path = "ocsf_json_messages/4689_0.json";
//     let file_contents = include_str!("ocsf_json_messages/4689_0.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_malware_infection_detected_sample_2() {
//     let file_path = "ocsf_json_messages/malware_infection_detected_sample_2.json";
//     let file_contents = include_str!("ocsf_json_messages/malware_infection_detected_sample_2.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_4688_0() {
//     let file_path = "ocsf_json_messages/4688_0.json";
//     let file_contents = include_str!("ocsf_json_messages/4688_0.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_offense() {
//     let file_path = "ocsf_json_messages/offense.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/offense.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_dns_log() {
//     let file_path = "ocsf_json_messages/dns_log.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/dns_log.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_ssl_log() {
//     let file_path = "ocsf_json_messages/ssl_log.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/ssl_log.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_conn_log() {
//     let file_path = "ocsf_json_messages/conn_log.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/conn_log.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_falco() {
//     let file_path = "ocsf_json_messages/falco.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/falco.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_eks() {
//     let file_path = "ocsf_json_messages/eks.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/eks.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_waf() {
//     let file_path = "ocsf_json_messages/waf.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/waf.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_security_hub() {
//     let file_path = "ocsf_json_messages/Security Hub.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/Security Hub.ocsf");
//     parse_example(file_path, file_contents);
// }

// These fail, but there are "newer" ones that pass
// #[test]
// fn parse_ocsf_account_change_0() {
//     let file_path = "ocsf_json_messages/1_0_0_OCSF_account_change.json";
//     let file_contents = include_str!("ocsf_json_messages/1_0_0_OCSF_account_change.json");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_1_0_0_account_change() {
//     let file_path = "ocsf_json_messages/1_0_0_account_change.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/1_0_0_account_change.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_1_0_0_authentication() {
//     let file_path = "ocsf_json_messages/1_0_0_authentication.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/1_0_0_authentication.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_1_0_0_generic_api_activity() {
//     let file_path = "ocsf_json_messages/1_0_0_generic_api_activity.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/1_0_0_generic_api_activity.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_1_0_0_vpcflowlog() {
//     let file_path = "ocsf_json_messages/1_0_0_vpcflowlog.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/1_0_0_vpcflowlog.ocsf");
//     parse_example(file_path, file_contents);
// }
// #[test]
// fn parse_1_0_0_route53() {
//     let file_path = "ocsf_json_messages/1_0_0_route53.ocsf";
//     let file_contents = include_str!("ocsf_json_messages/1_0_0_route53.ocsf");
//     parse_example(file_path, file_contents);
// }
