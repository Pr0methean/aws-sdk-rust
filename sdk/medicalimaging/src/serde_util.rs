// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn access_denied_exception_correct_errors(
    mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
) -> crate::types::error::builders::AccessDeniedExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn conflict_exception_correct_errors(
    mut builder: crate::types::error::builders::ConflictExceptionBuilder,
) -> crate::types::error::builders::ConflictExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn internal_server_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
) -> crate::types::error::builders::InternalServerExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn resource_not_found_exception_correct_errors(
    mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
) -> crate::types::error::builders::ResourceNotFoundExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn service_quota_exceeded_exception_correct_errors(
    mut builder: crate::types::error::builders::ServiceQuotaExceededExceptionBuilder,
) -> crate::types::error::builders::ServiceQuotaExceededExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn throttling_exception_correct_errors(
    mut builder: crate::types::error::builders::ThrottlingExceptionBuilder,
) -> crate::types::error::builders::ThrottlingExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_correct_errors(
    mut builder: crate::types::error::builders::ValidationExceptionBuilder,
) -> crate::types::error::builders::ValidationExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn copy_image_set_output_output_correct_errors(
    mut builder: crate::operation::copy_image_set::builders::CopyImageSetOutputBuilder,
) -> crate::operation::copy_image_set::builders::CopyImageSetOutputBuilder {
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.source_image_set_properties.is_none() {
        builder.source_image_set_properties = {
            let builder = crate::types::builders::CopySourceImageSetPropertiesBuilder::default();
            crate::serde_util::copy_source_image_set_properties_correct_errors(builder).build().ok()
        }
    }
    if builder.destination_image_set_properties.is_none() {
        builder.destination_image_set_properties = {
            let builder = crate::types::builders::CopyDestinationImageSetPropertiesBuilder::default();
            crate::serde_util::copy_destination_image_set_properties_correct_errors(builder)
                .build()
                .ok()
        }
    }
    builder
}

pub(crate) fn create_datastore_output_output_correct_errors(
    mut builder: crate::operation::create_datastore::builders::CreateDatastoreOutputBuilder,
) -> crate::operation::create_datastore::builders::CreateDatastoreOutputBuilder {
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.datastore_status.is_none() {
        builder.datastore_status = "no value was set".parse::<crate::types::DatastoreStatus>().ok()
    }
    builder
}

pub(crate) fn delete_datastore_output_output_correct_errors(
    mut builder: crate::operation::delete_datastore::builders::DeleteDatastoreOutputBuilder,
) -> crate::operation::delete_datastore::builders::DeleteDatastoreOutputBuilder {
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.datastore_status.is_none() {
        builder.datastore_status = "no value was set".parse::<crate::types::DatastoreStatus>().ok()
    }
    builder
}

pub(crate) fn delete_image_set_output_output_correct_errors(
    mut builder: crate::operation::delete_image_set::builders::DeleteImageSetOutputBuilder,
) -> crate::operation::delete_image_set::builders::DeleteImageSetOutputBuilder {
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.image_set_id.is_none() {
        builder.image_set_id = Some(Default::default())
    }
    if builder.image_set_state.is_none() {
        builder.image_set_state = "no value was set".parse::<crate::types::ImageSetState>().ok()
    }
    if builder.image_set_workflow_status.is_none() {
        builder.image_set_workflow_status = "no value was set".parse::<crate::types::ImageSetWorkflowStatus>().ok()
    }
    builder
}

pub(crate) fn get_datastore_output_output_correct_errors(
    mut builder: crate::operation::get_datastore::builders::GetDatastoreOutputBuilder,
) -> crate::operation::get_datastore::builders::GetDatastoreOutputBuilder {
    if builder.datastore_properties.is_none() {
        builder.datastore_properties = {
            let builder = crate::types::builders::DatastorePropertiesBuilder::default();
            crate::serde_util::datastore_properties_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn get_dicom_import_job_output_output_correct_errors(
    mut builder: crate::operation::get_dicom_import_job::builders::GetDicomImportJobOutputBuilder,
) -> crate::operation::get_dicom_import_job::builders::GetDicomImportJobOutputBuilder {
    if builder.job_properties.is_none() {
        builder.job_properties = {
            let builder = crate::types::builders::DicomImportJobPropertiesBuilder::default();
            crate::serde_util::dicom_import_job_properties_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn get_image_set_output_output_correct_errors(
    mut builder: crate::operation::get_image_set::builders::GetImageSetOutputBuilder,
) -> crate::operation::get_image_set::builders::GetImageSetOutputBuilder {
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.image_set_id.is_none() {
        builder.image_set_id = Some(Default::default())
    }
    if builder.version_id.is_none() {
        builder.version_id = Some(Default::default())
    }
    if builder.image_set_state.is_none() {
        builder.image_set_state = "no value was set".parse::<crate::types::ImageSetState>().ok()
    }
    builder
}

pub(crate) fn list_dicom_import_jobs_output_output_correct_errors(
    mut builder: crate::operation::list_dicom_import_jobs::builders::ListDicomImportJobsOutputBuilder,
) -> crate::operation::list_dicom_import_jobs::builders::ListDicomImportJobsOutputBuilder {
    if builder.job_summaries.is_none() {
        builder.job_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn list_image_set_versions_output_output_correct_errors(
    mut builder: crate::operation::list_image_set_versions::builders::ListImageSetVersionsOutputBuilder,
) -> crate::operation::list_image_set_versions::builders::ListImageSetVersionsOutputBuilder {
    if builder.image_set_properties_list.is_none() {
        builder.image_set_properties_list = Some(Default::default())
    }
    builder
}

pub(crate) fn list_tags_for_resource_output_output_correct_errors(
    mut builder: crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder,
) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder {
    if builder.tags.is_none() {
        builder.tags = Some(Default::default())
    }
    builder
}

pub(crate) fn search_image_sets_output_output_correct_errors(
    mut builder: crate::operation::search_image_sets::builders::SearchImageSetsOutputBuilder,
) -> crate::operation::search_image_sets::builders::SearchImageSetsOutputBuilder {
    if builder.image_sets_metadata_summaries.is_none() {
        builder.image_sets_metadata_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn start_dicom_import_job_output_output_correct_errors(
    mut builder: crate::operation::start_dicom_import_job::builders::StartDicomImportJobOutputBuilder,
) -> crate::operation::start_dicom_import_job::builders::StartDicomImportJobOutputBuilder {
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.job_id.is_none() {
        builder.job_id = Some(Default::default())
    }
    if builder.job_status.is_none() {
        builder.job_status = "no value was set".parse::<crate::types::JobStatus>().ok()
    }
    if builder.submitted_at.is_none() {
        builder.submitted_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn update_image_set_metadata_output_output_correct_errors(
    mut builder: crate::operation::update_image_set_metadata::builders::UpdateImageSetMetadataOutputBuilder,
) -> crate::operation::update_image_set_metadata::builders::UpdateImageSetMetadataOutputBuilder {
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.image_set_id.is_none() {
        builder.image_set_id = Some(Default::default())
    }
    if builder.latest_version_id.is_none() {
        builder.latest_version_id = Some(Default::default())
    }
    if builder.image_set_state.is_none() {
        builder.image_set_state = "no value was set".parse::<crate::types::ImageSetState>().ok()
    }
    builder
}

pub(crate) fn copy_source_image_set_properties_correct_errors(
    mut builder: crate::types::builders::CopySourceImageSetPropertiesBuilder,
) -> crate::types::builders::CopySourceImageSetPropertiesBuilder {
    if builder.image_set_id.is_none() {
        builder.image_set_id = Some(Default::default())
    }
    if builder.latest_version_id.is_none() {
        builder.latest_version_id = Some(Default::default())
    }
    builder
}

pub(crate) fn copy_destination_image_set_properties_correct_errors(
    mut builder: crate::types::builders::CopyDestinationImageSetPropertiesBuilder,
) -> crate::types::builders::CopyDestinationImageSetPropertiesBuilder {
    if builder.image_set_id.is_none() {
        builder.image_set_id = Some(Default::default())
    }
    if builder.latest_version_id.is_none() {
        builder.latest_version_id = Some(Default::default())
    }
    builder
}

pub(crate) fn datastore_properties_correct_errors(
    mut builder: crate::types::builders::DatastorePropertiesBuilder,
) -> crate::types::builders::DatastorePropertiesBuilder {
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.datastore_name.is_none() {
        builder.datastore_name = Some(Default::default())
    }
    if builder.datastore_status.is_none() {
        builder.datastore_status = "no value was set".parse::<crate::types::DatastoreStatus>().ok()
    }
    builder
}

pub(crate) fn dicom_import_job_properties_correct_errors(
    mut builder: crate::types::builders::DicomImportJobPropertiesBuilder,
) -> crate::types::builders::DicomImportJobPropertiesBuilder {
    if builder.job_id.is_none() {
        builder.job_id = Some(Default::default())
    }
    if builder.job_name.is_none() {
        builder.job_name = Some(Default::default())
    }
    if builder.job_status.is_none() {
        builder.job_status = "no value was set".parse::<crate::types::JobStatus>().ok()
    }
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.data_access_role_arn.is_none() {
        builder.data_access_role_arn = Some(Default::default())
    }
    if builder.input_s3_uri.is_none() {
        builder.input_s3_uri = Some(Default::default())
    }
    if builder.output_s3_uri.is_none() {
        builder.output_s3_uri = Some(Default::default())
    }
    builder
}

pub(crate) fn datastore_summary_correct_errors(
    mut builder: crate::types::builders::DatastoreSummaryBuilder,
) -> crate::types::builders::DatastoreSummaryBuilder {
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    if builder.datastore_name.is_none() {
        builder.datastore_name = Some(Default::default())
    }
    if builder.datastore_status.is_none() {
        builder.datastore_status = "no value was set".parse::<crate::types::DatastoreStatus>().ok()
    }
    builder
}

pub(crate) fn dicom_import_job_summary_correct_errors(
    mut builder: crate::types::builders::DicomImportJobSummaryBuilder,
) -> crate::types::builders::DicomImportJobSummaryBuilder {
    if builder.job_id.is_none() {
        builder.job_id = Some(Default::default())
    }
    if builder.job_name.is_none() {
        builder.job_name = Some(Default::default())
    }
    if builder.job_status.is_none() {
        builder.job_status = "no value was set".parse::<crate::types::JobStatus>().ok()
    }
    if builder.datastore_id.is_none() {
        builder.datastore_id = Some(Default::default())
    }
    builder
}

pub(crate) fn image_set_properties_correct_errors(
    mut builder: crate::types::builders::ImageSetPropertiesBuilder,
) -> crate::types::builders::ImageSetPropertiesBuilder {
    if builder.image_set_id.is_none() {
        builder.image_set_id = Some(Default::default())
    }
    if builder.version_id.is_none() {
        builder.version_id = Some(Default::default())
    }
    if builder.image_set_state.is_none() {
        builder.image_set_state = "no value was set".parse::<crate::types::ImageSetState>().ok()
    }
    builder
}

pub(crate) fn image_sets_metadata_summary_correct_errors(
    mut builder: crate::types::builders::ImageSetsMetadataSummaryBuilder,
) -> crate::types::builders::ImageSetsMetadataSummaryBuilder {
    if builder.image_set_id.is_none() {
        builder.image_set_id = Some(Default::default())
    }
    builder
}
