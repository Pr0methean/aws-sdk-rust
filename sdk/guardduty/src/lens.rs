// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_malware_scans_output_next_token(
    input: &crate::output::DescribeMalwareScansOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_get_usage_statistics_output_next_token(
    input: &crate::output::GetUsageStatisticsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_detectors_output_next_token(
    input: &crate::output::ListDetectorsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_filters_output_next_token(
    input: &crate::output::ListFiltersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_findings_output_next_token(
    input: &crate::output::ListFindingsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_invitations_output_next_token(
    input: &crate::output::ListInvitationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_ip_sets_output_next_token(
    input: &crate::output::ListIpSetsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_members_output_next_token(
    input: &crate::output::ListMembersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_organization_admin_accounts_output_next_token(
    input: &crate::output::ListOrganizationAdminAccountsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_publishing_destinations_output_next_token(
    input: &crate::output::ListPublishingDestinationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_threat_intel_sets_output_next_token(
    input: &crate::output::ListThreatIntelSetsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_malware_scans_output_scans(
    input: crate::output::DescribeMalwareScansOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Scan>> {
    let input = match input.scans {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_detectors_output_detector_ids(
    input: crate::output::ListDetectorsOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.detector_ids {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_filters_output_filter_names(
    input: crate::output::ListFiltersOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.filter_names {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_findings_output_finding_ids(
    input: crate::output::ListFindingsOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.finding_ids {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_invitations_output_invitations(
    input: crate::output::ListInvitationsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Invitation>> {
    let input = match input.invitations {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_ip_sets_output_ip_set_ids(
    input: crate::output::ListIpSetsOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.ip_set_ids {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_members_output_members(
    input: crate::output::ListMembersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Member>> {
    let input = match input.members {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_organization_admin_accounts_output_admin_accounts(
    input: crate::output::ListOrganizationAdminAccountsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::AdminAccount>> {
    let input = match input.admin_accounts {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_threat_intel_sets_output_threat_intel_set_ids(
    input: crate::output::ListThreatIntelSetsOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.threat_intel_set_ids {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
