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

use crate::struct_fields::IntoILFAttributes;
use enum_common_fields::EnumCommonFields;
use ilf_ocsf_macro::ParseFromUid;
use ocsf::Metadata;
use protobuf::MessageFull;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

/// Wrapper function for the protobuf_json_mapping::parse_from_str method to coerce the error type
fn _parse_wrapper<M: MessageFull>(json: &str) -> Result<M, Box<dyn std::error::Error>> {
    protobuf_json_mapping::parse_from_str::<M>(json).map_err(|e| e.into())
}

/// An Enum variant for every type of OCSF Category, tagged with a uid so we can dynamically parse them as they come in.
#[derive(ParseFromUid, Debug, EnumCommonFields)]
#[Parser(_parse_wrapper)]
#[common_field(type_uid: i64)]
#[common_field(metadata: Metadata)]
pub enum OCSFEvent {
    // SystemActivity
    #[Uid(1001)]
    FileActivity(ocsf::FileActivity),
    #[Uid(201001)]
    RegistryKeyActivity(ocsf::RegistryKeyActivity),
    #[Uid(201002)]
    RegistryValueActivity(ocsf::RegistryValueActivity),
    #[Uid(1002)]
    KernelExtension(ocsf::KernelExtension),
    #[Uid(1003)]
    KernelActivity(ocsf::KernelActivity),
    #[Uid(20100)]
    ResourceActivity(ocsf::ResourceActivity),
    #[Uid(1004)]
    MemoryActivity(ocsf::MemoryActivity),
    #[Uid(1005)]
    ModuleActivity(ocsf::ModuleActivity),
    #[Uid(1006)]
    ScheduledJobActivity(ocsf::ScheduledJobActivity),
    #[Uid(1007)]
    ProcessActivity(ocsf::ProcessActivity),
    // Findings
    #[Uid(2001)]
    SecurityFinding(ocsf::SecurityFinding),
    #[Uid(2002)]
    VulnerabilityFinding(ocsf::VulnerabilityFinding),
    #[Uid(2003)]
    ComplianceFinding(ocsf::ComplianceFinding),
    #[Uid(2004)]
    DetectionFinding(ocsf::DetectionFinding),
    #[Uid(2005)]
    IncidentFinding(ocsf::IncidentFinding),
    #[Uid(2006)]
    DataSecurityFinding(ocsf::DataSecurityFinding),
    // IdentityAndAccessManagement
    #[Uid(3001)]
    AccountChange(ocsf::AccountChange),
    #[Uid(3002)]
    Authentication(ocsf::Authentication),
    #[Uid(3003)]
    AuthorizeSession(ocsf::AuthorizeSession),
    #[Uid(3004)]
    EntityManagement(ocsf::EntityManagement),
    #[Uid(3005)]
    UserAccess(ocsf::UserAccess),
    #[Uid(3006)]
    GroupManagement(ocsf::GroupManagement),
    // NetworkActivity
    #[Uid(4001)]
    NetworkActivity(ocsf::NetworkActivity),
    #[Uid(4002)]
    HttpActivity(ocsf::HttpActivity),
    #[Uid(4003)]
    DnsActivity(ocsf::DnsActivity),
    #[Uid(4004)]
    DhcpActivity(ocsf::DhcpActivity),
    #[Uid(4005)]
    RdpActivity(ocsf::RdpActivity),
    #[Uid(4006)]
    SmbActivity(ocsf::SmbActivity),
    #[Uid(4007)]
    SshActivity(ocsf::SshActivity),
    #[Uid(4008)]
    FtpActivity(ocsf::FtpActivity),
    #[Uid(4009)]
    EmailActivity(ocsf::EmailActivity),
    #[Uid(4010)]
    NetworkFileActivity(ocsf::NetworkFileActivity),
    #[Uid(4011)]
    EmailFileActivity(ocsf::EmailFileActivity),
    #[Uid(4012)]
    EmailUrlActivity(ocsf::EmailUrlActivity),
    #[Uid(4013)]
    NtpActivity(ocsf::NtpActivity),
    #[Uid(4014)]
    TunnelActivity(ocsf::TunnelActivity),
    // Discovery
    #[Uid(5001)]
    InventoryInfo(ocsf::InventoryInfo),
    #[Uid(5002)]
    ConfigState(ocsf::ConfigState),
    #[Uid(5003)]
    UserInventory(ocsf::UserInventory),
    #[Uid(500)]
    PatchState(ocsf::PatchState),
    #[Uid(205004)]
    RegistryKeyQuery(ocsf::RegistryKeyQuery),
    #[Uid(205005)]
    RegistryValueQuery(ocsf::RegistryValueQuery),
    #[Uid(5006)]
    KernelObjectQuery(ocsf::KernelObjectQuery),
    #[Uid(5007)]
    FileQuery(ocsf::FileQuery),
    #[Uid(5008)]
    FolderQuery(ocsf::FolderQuery),
    #[Uid(5009)]
    AdminGroupQuery(ocsf::AdminGroupQuery),
    #[Uid(5010)]
    JobQuery(ocsf::JobQuery),
    #[Uid(5011)]
    ModuleQuery(ocsf::ModuleQuery),
    #[Uid(5012)]
    NetworkConnectionQuery(ocsf::NetworkConnectionQuery),
    #[Uid(5013)]
    NetworksQuery(ocsf::NetworksQuery),
    #[Uid(5014)]
    PeripheralDeviceQuery(ocsf::PeripheralDeviceQuery),
    #[Uid(5015)]
    ProcessQuery(ocsf::ProcessQuery),
    #[Uid(5016)]
    ServiceQuery(ocsf::ServiceQuery),
    #[Uid(5017)]
    SessionQuery(ocsf::SessionQuery),
    #[Uid(5018)]
    UserQuery(ocsf::UserQuery),
    #[Uid(5019)]
    DeviceConfigStateChange(ocsf::DeviceConfigStateChange),
    #[Uid(205019)]
    PrefetchQuery(ocsf::PrefetchQuery),
    // ApplicationActivity
    #[Uid(6001)]
    WebResourcesActivity(ocsf::WebResourcesActivity),
    #[Uid(6002)]
    ApplicationLifecycle(ocsf::ApplicationLifecycle),
    #[Uid(6003)]
    ApiActivity(ocsf::ApiActivity),
    #[Uid(600)]
    WebResourceAccessActivity(ocsf::WebResourceAccessActivity),
    #[Uid(6005)]
    DatastoreActivity(ocsf::DatastoreActivity),
    #[Uid(6006)]
    FileHosting(ocsf::FileHosting),
    #[Uid(6007)]
    ScanActivity(ocsf::ScanActivity),
    // TO ADD?
    // Event Log Activity [1008] ????
    // Operating System Patch State [5004] ??
    // Software Inventory Info [5020] ??
    // Web Resource Access Activity [6004] ???
}

impl OCSFEvent {
    pub fn get_vendor_name(&self) -> &str {
        &self.metadata().product.vendor_name
    }

    pub fn to_ilf<'b>(
        &self,
        include_unmapped: bool,
        include_missing_and_none: bool,
    ) -> Result<seal_lib::Log, Box<dyn std::error::Error>> {
        let path = String::new();
        let mut store = Vec::new();
        let ilf_name = format!("OCSF_{}", self.type_uid());

        self.into_ilf_attributes(path, &mut store);
        let source = self.get_vendor_name().to_string();

        let filtered_null = store
            .into_iter()
            .filter(|(_, value)| {
                include_missing_and_none || !matches!(value, seal_lib::Value::VNone)
            })
            .filter(|(path, _)| include_unmapped || !path.starts_with("unmapped"))
            .collect();

        Ok(seal_lib::Log::new_from_attributes(
            ilf_name,
            Some(source),
            None,
            None,
            vec![],
            filtered_null,
        ))
    }

    // TODO: you could handle the into_ilf_attributes call with a clever common_method marco (similar to the
    //  common_field macro used above). But until then any update to the OCSFEvent struct needs to be reflected in the
    //  below match statement.
    pub fn into_ilf_attributes(
        &self,
        path: String,
        mut store: &mut Vec<(String, seal_lib::Value)>,
    ) {
        match self {
            OCSFEvent::FileActivity(file_activity) => {
                file_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::RegistryKeyActivity(registry_key_activity) => {
                registry_key_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::RegistryValueActivity(registry_value_activity) => {
                registry_value_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::KernelExtension(kernel_extension) => {
                kernel_extension.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::KernelActivity(kernel_activity) => {
                kernel_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ResourceActivity(resource_activity) => {
                resource_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::MemoryActivity(memory_activity) => {
                memory_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ModuleActivity(module_activity) => {
                module_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ScheduledJobActivity(scheduled_job_activity) => {
                scheduled_job_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ProcessActivity(process_activity) => {
                process_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::SecurityFinding(security_finding) => {
                security_finding.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::VulnerabilityFinding(vulnerability_finding) => {
                vulnerability_finding.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ComplianceFinding(compliance_finding) => {
                compliance_finding.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::DetectionFinding(detection_finding) => {
                detection_finding.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::IncidentFinding(incident_finding) => {
                incident_finding.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::DataSecurityFinding(data_security_finding) => {
                data_security_finding.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::AccountChange(account_change) => {
                account_change.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::Authentication(authentication) => {
                authentication.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::AuthorizeSession(authorize_session) => {
                authorize_session.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::EntityManagement(entity_management) => {
                entity_management.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::UserAccess(user_access) => user_access.into_ilf_attributes(path, &mut store),
            OCSFEvent::GroupManagement(group_management) => {
                group_management.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::NetworkActivity(network_activity) => {
                network_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::HttpActivity(http_activity) => {
                http_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::DnsActivity(dns_activity) => {
                dns_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::DhcpActivity(dhcp_activity) => {
                dhcp_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::RdpActivity(rdp_activity) => {
                rdp_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::SmbActivity(smb_activity) => {
                smb_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::SshActivity(ssh_activity) => {
                ssh_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::FtpActivity(ftp_activity) => {
                ftp_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::EmailActivity(email_activity) => {
                email_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::NetworkFileActivity(network_file_activity) => {
                network_file_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::EmailFileActivity(email_file_activity) => {
                email_file_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::EmailUrlActivity(email_url_activity) => {
                email_url_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::NtpActivity(ntp_activity) => {
                ntp_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::TunnelActivity(tunnel_activity) => {
                tunnel_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::InventoryInfo(inventory_info) => {
                inventory_info.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ConfigState(config_state) => {
                config_state.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::UserInventory(user_inventory) => {
                user_inventory.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::PatchState(patch_state) => patch_state.into_ilf_attributes(path, &mut store),
            OCSFEvent::RegistryKeyQuery(registry_key_query) => {
                registry_key_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::RegistryValueQuery(registry_value_query) => {
                registry_value_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::KernelObjectQuery(kernel_object_query) => {
                kernel_object_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::FileQuery(file_query) => file_query.into_ilf_attributes(path, &mut store),
            OCSFEvent::FolderQuery(folder_query) => {
                folder_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::AdminGroupQuery(admin_group_query) => {
                admin_group_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::JobQuery(job_query) => job_query.into_ilf_attributes(path, &mut store),
            OCSFEvent::ModuleQuery(module_query) => {
                module_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::NetworkConnectionQuery(network_connection_query) => {
                network_connection_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::NetworksQuery(networks_query) => {
                networks_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::PeripheralDeviceQuery(peripheral_device_query) => {
                peripheral_device_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ProcessQuery(process_query) => {
                process_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ServiceQuery(service_query) => {
                service_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::SessionQuery(session_query) => {
                session_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::UserQuery(user_query) => user_query.into_ilf_attributes(path, &mut store),
            OCSFEvent::DeviceConfigStateChange(device_config_state_change) => {
                device_config_state_change.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::PrefetchQuery(prefetch_query) => {
                prefetch_query.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::WebResourcesActivity(web_resources_activity) => {
                web_resources_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ApplicationLifecycle(application_lifecycle) => {
                application_lifecycle.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ApiActivity(api_activity) => {
                api_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::WebResourceAccessActivity(web_resource_access_activity) => {
                web_resource_access_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::DatastoreActivity(datastore_activity) => {
                datastore_activity.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::FileHosting(file_hosting) => {
                file_hosting.into_ilf_attributes(path, &mut store)
            }
            OCSFEvent::ScanActivity(scan_activity) => {
                scan_activity.into_ilf_attributes(path, &mut store)
            }
        }
    }
}
