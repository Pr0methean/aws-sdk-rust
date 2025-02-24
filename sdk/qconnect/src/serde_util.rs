// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn get_recommendations_output_output_correct_errors(
    mut builder: crate::operation::get_recommendations::builders::GetRecommendationsOutputBuilder,
) -> crate::operation::get_recommendations::builders::GetRecommendationsOutputBuilder {
    if builder.recommendations.is_none() {
        builder.recommendations = Some(Default::default())
    }
    builder
}

pub(crate) fn list_assistant_associations_output_output_correct_errors(
    mut builder: crate::operation::list_assistant_associations::builders::ListAssistantAssociationsOutputBuilder,
) -> crate::operation::list_assistant_associations::builders::ListAssistantAssociationsOutputBuilder {
    if builder.assistant_association_summaries.is_none() {
        builder.assistant_association_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn list_assistants_output_output_correct_errors(
    mut builder: crate::operation::list_assistants::builders::ListAssistantsOutputBuilder,
) -> crate::operation::list_assistants::builders::ListAssistantsOutputBuilder {
    if builder.assistant_summaries.is_none() {
        builder.assistant_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn list_contents_output_output_correct_errors(
    mut builder: crate::operation::list_contents::builders::ListContentsOutputBuilder,
) -> crate::operation::list_contents::builders::ListContentsOutputBuilder {
    if builder.content_summaries.is_none() {
        builder.content_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn list_import_jobs_output_output_correct_errors(
    mut builder: crate::operation::list_import_jobs::builders::ListImportJobsOutputBuilder,
) -> crate::operation::list_import_jobs::builders::ListImportJobsOutputBuilder {
    if builder.import_job_summaries.is_none() {
        builder.import_job_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn list_knowledge_bases_output_output_correct_errors(
    mut builder: crate::operation::list_knowledge_bases::builders::ListKnowledgeBasesOutputBuilder,
) -> crate::operation::list_knowledge_bases::builders::ListKnowledgeBasesOutputBuilder {
    if builder.knowledge_base_summaries.is_none() {
        builder.knowledge_base_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn list_quick_responses_output_output_correct_errors(
    mut builder: crate::operation::list_quick_responses::builders::ListQuickResponsesOutputBuilder,
) -> crate::operation::list_quick_responses::builders::ListQuickResponsesOutputBuilder {
    if builder.quick_response_summaries.is_none() {
        builder.quick_response_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn put_feedback_output_output_correct_errors(
    mut builder: crate::operation::put_feedback::builders::PutFeedbackOutputBuilder,
) -> crate::operation::put_feedback::builders::PutFeedbackOutputBuilder {
    if builder.assistant_id.is_none() {
        builder.assistant_id = Some(Default::default())
    }
    if builder.assistant_arn.is_none() {
        builder.assistant_arn = Some(Default::default())
    }
    if builder.target_id.is_none() {
        builder.target_id = Some(Default::default())
    }
    if builder.target_type.is_none() {
        builder.target_type = "no value was set".parse::<crate::types::TargetType>().ok()
    }
    if builder.content_feedback.is_none() {
        builder.content_feedback = Some(crate::types::ContentFeedbackData::Unknown)
    }
    builder
}

pub(crate) fn query_assistant_output_output_correct_errors(
    mut builder: crate::operation::query_assistant::builders::QueryAssistantOutputBuilder,
) -> crate::operation::query_assistant::builders::QueryAssistantOutputBuilder {
    if builder.results.is_none() {
        builder.results = Some(Default::default())
    }
    builder
}

pub(crate) fn search_content_output_output_correct_errors(
    mut builder: crate::operation::search_content::builders::SearchContentOutputBuilder,
) -> crate::operation::search_content::builders::SearchContentOutputBuilder {
    if builder.content_summaries.is_none() {
        builder.content_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn search_quick_responses_output_output_correct_errors(
    mut builder: crate::operation::search_quick_responses::builders::SearchQuickResponsesOutputBuilder,
) -> crate::operation::search_quick_responses::builders::SearchQuickResponsesOutputBuilder {
    if builder.results.is_none() {
        builder.results = Some(Default::default())
    }
    builder
}

pub(crate) fn search_sessions_output_output_correct_errors(
    mut builder: crate::operation::search_sessions::builders::SearchSessionsOutputBuilder,
) -> crate::operation::search_sessions::builders::SearchSessionsOutputBuilder {
    if builder.session_summaries.is_none() {
        builder.session_summaries = Some(Default::default())
    }
    builder
}

pub(crate) fn start_content_upload_output_output_correct_errors(
    mut builder: crate::operation::start_content_upload::builders::StartContentUploadOutputBuilder,
) -> crate::operation::start_content_upload::builders::StartContentUploadOutputBuilder {
    if builder.upload_id.is_none() {
        builder.upload_id = Some(Default::default())
    }
    if builder.url.is_none() {
        builder.url = Some(Default::default())
    }
    if builder.url_expiry.is_none() {
        builder.url_expiry = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.headers_to_include.is_none() {
        builder.headers_to_include = Some(Default::default())
    }
    builder
}

pub(crate) fn assistant_association_data_correct_errors(
    mut builder: crate::types::builders::AssistantAssociationDataBuilder,
) -> crate::types::builders::AssistantAssociationDataBuilder {
    if builder.assistant_association_id.is_none() {
        builder.assistant_association_id = Some(Default::default())
    }
    if builder.assistant_association_arn.is_none() {
        builder.assistant_association_arn = Some(Default::default())
    }
    if builder.assistant_id.is_none() {
        builder.assistant_id = Some(Default::default())
    }
    if builder.assistant_arn.is_none() {
        builder.assistant_arn = Some(Default::default())
    }
    if builder.association_type.is_none() {
        builder.association_type = "no value was set".parse::<crate::types::AssociationType>().ok()
    }
    if builder.association_data.is_none() {
        builder.association_data = Some(crate::types::AssistantAssociationOutputData::Unknown)
    }
    builder
}

pub(crate) fn assistant_data_correct_errors(
    mut builder: crate::types::builders::AssistantDataBuilder,
) -> crate::types::builders::AssistantDataBuilder {
    if builder.assistant_id.is_none() {
        builder.assistant_id = Some(Default::default())
    }
    if builder.assistant_arn.is_none() {
        builder.assistant_arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::AssistantType>().ok()
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::AssistantStatus>().ok()
    }
    builder
}

pub(crate) fn content_data_correct_errors(mut builder: crate::types::builders::ContentDataBuilder) -> crate::types::builders::ContentDataBuilder {
    if builder.content_arn.is_none() {
        builder.content_arn = Some(Default::default())
    }
    if builder.content_id.is_none() {
        builder.content_id = Some(Default::default())
    }
    if builder.knowledge_base_arn.is_none() {
        builder.knowledge_base_arn = Some(Default::default())
    }
    if builder.knowledge_base_id.is_none() {
        builder.knowledge_base_id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.revision_id.is_none() {
        builder.revision_id = Some(Default::default())
    }
    if builder.title.is_none() {
        builder.title = Some(Default::default())
    }
    if builder.content_type.is_none() {
        builder.content_type = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ContentStatus>().ok()
    }
    if builder.metadata.is_none() {
        builder.metadata = Some(Default::default())
    }
    if builder.url.is_none() {
        builder.url = Some(Default::default())
    }
    if builder.url_expiry.is_none() {
        builder.url_expiry = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn content_summary_correct_errors(
    mut builder: crate::types::builders::ContentSummaryBuilder,
) -> crate::types::builders::ContentSummaryBuilder {
    if builder.content_arn.is_none() {
        builder.content_arn = Some(Default::default())
    }
    if builder.content_id.is_none() {
        builder.content_id = Some(Default::default())
    }
    if builder.knowledge_base_arn.is_none() {
        builder.knowledge_base_arn = Some(Default::default())
    }
    if builder.knowledge_base_id.is_none() {
        builder.knowledge_base_id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.revision_id.is_none() {
        builder.revision_id = Some(Default::default())
    }
    if builder.title.is_none() {
        builder.title = Some(Default::default())
    }
    if builder.content_type.is_none() {
        builder.content_type = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ContentStatus>().ok()
    }
    if builder.metadata.is_none() {
        builder.metadata = Some(Default::default())
    }
    builder
}

pub(crate) fn import_job_data_correct_errors(
    mut builder: crate::types::builders::ImportJobDataBuilder,
) -> crate::types::builders::ImportJobDataBuilder {
    if builder.import_job_id.is_none() {
        builder.import_job_id = Some(Default::default())
    }
    if builder.knowledge_base_id.is_none() {
        builder.knowledge_base_id = Some(Default::default())
    }
    if builder.upload_id.is_none() {
        builder.upload_id = Some(Default::default())
    }
    if builder.knowledge_base_arn.is_none() {
        builder.knowledge_base_arn = Some(Default::default())
    }
    if builder.import_job_type.is_none() {
        builder.import_job_type = "no value was set".parse::<crate::types::ImportJobType>().ok()
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ImportJobStatus>().ok()
    }
    if builder.url.is_none() {
        builder.url = Some(Default::default())
    }
    if builder.url_expiry.is_none() {
        builder.url_expiry = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.created_time.is_none() {
        builder.created_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.last_modified_time.is_none() {
        builder.last_modified_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn knowledge_base_data_correct_errors(
    mut builder: crate::types::builders::KnowledgeBaseDataBuilder,
) -> crate::types::builders::KnowledgeBaseDataBuilder {
    if builder.knowledge_base_id.is_none() {
        builder.knowledge_base_id = Some(Default::default())
    }
    if builder.knowledge_base_arn.is_none() {
        builder.knowledge_base_arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.knowledge_base_type.is_none() {
        builder.knowledge_base_type = "no value was set".parse::<crate::types::KnowledgeBaseType>().ok()
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::KnowledgeBaseStatus>().ok()
    }
    builder
}

pub(crate) fn quick_response_data_correct_errors(
    mut builder: crate::types::builders::QuickResponseDataBuilder,
) -> crate::types::builders::QuickResponseDataBuilder {
    if builder.quick_response_arn.is_none() {
        builder.quick_response_arn = Some(Default::default())
    }
    if builder.quick_response_id.is_none() {
        builder.quick_response_id = Some(Default::default())
    }
    if builder.knowledge_base_arn.is_none() {
        builder.knowledge_base_arn = Some(Default::default())
    }
    if builder.knowledge_base_id.is_none() {
        builder.knowledge_base_id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.content_type.is_none() {
        builder.content_type = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::QuickResponseStatus>().ok()
    }
    if builder.created_time.is_none() {
        builder.created_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.last_modified_time.is_none() {
        builder.last_modified_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn session_data_correct_errors(mut builder: crate::types::builders::SessionDataBuilder) -> crate::types::builders::SessionDataBuilder {
    if builder.session_arn.is_none() {
        builder.session_arn = Some(Default::default())
    }
    if builder.session_id.is_none() {
        builder.session_id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn assistant_association_summary_correct_errors(
    mut builder: crate::types::builders::AssistantAssociationSummaryBuilder,
) -> crate::types::builders::AssistantAssociationSummaryBuilder {
    if builder.assistant_association_id.is_none() {
        builder.assistant_association_id = Some(Default::default())
    }
    if builder.assistant_association_arn.is_none() {
        builder.assistant_association_arn = Some(Default::default())
    }
    if builder.assistant_id.is_none() {
        builder.assistant_id = Some(Default::default())
    }
    if builder.assistant_arn.is_none() {
        builder.assistant_arn = Some(Default::default())
    }
    if builder.association_type.is_none() {
        builder.association_type = "no value was set".parse::<crate::types::AssociationType>().ok()
    }
    if builder.association_data.is_none() {
        builder.association_data = Some(crate::types::AssistantAssociationOutputData::Unknown)
    }
    builder
}

pub(crate) fn assistant_summary_correct_errors(
    mut builder: crate::types::builders::AssistantSummaryBuilder,
) -> crate::types::builders::AssistantSummaryBuilder {
    if builder.assistant_id.is_none() {
        builder.assistant_id = Some(Default::default())
    }
    if builder.assistant_arn.is_none() {
        builder.assistant_arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::AssistantType>().ok()
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::AssistantStatus>().ok()
    }
    builder
}

pub(crate) fn external_source_configuration_correct_errors(
    mut builder: crate::types::builders::ExternalSourceConfigurationBuilder,
) -> crate::types::builders::ExternalSourceConfigurationBuilder {
    if builder.source.is_none() {
        builder.source = "no value was set".parse::<crate::types::ExternalSource>().ok()
    }
    if builder.configuration.is_none() {
        builder.configuration = Some(crate::types::Configuration::Unknown)
    }
    builder
}

pub(crate) fn generative_content_feedback_data_correct_errors(
    mut builder: crate::types::builders::GenerativeContentFeedbackDataBuilder,
) -> crate::types::builders::GenerativeContentFeedbackDataBuilder {
    if builder.relevance.is_none() {
        builder.relevance = "no value was set".parse::<crate::types::Relevance>().ok()
    }
    builder
}

pub(crate) fn import_job_summary_correct_errors(
    mut builder: crate::types::builders::ImportJobSummaryBuilder,
) -> crate::types::builders::ImportJobSummaryBuilder {
    if builder.import_job_id.is_none() {
        builder.import_job_id = Some(Default::default())
    }
    if builder.knowledge_base_id.is_none() {
        builder.knowledge_base_id = Some(Default::default())
    }
    if builder.upload_id.is_none() {
        builder.upload_id = Some(Default::default())
    }
    if builder.knowledge_base_arn.is_none() {
        builder.knowledge_base_arn = Some(Default::default())
    }
    if builder.import_job_type.is_none() {
        builder.import_job_type = "no value was set".parse::<crate::types::ImportJobType>().ok()
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ImportJobStatus>().ok()
    }
    if builder.created_time.is_none() {
        builder.created_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.last_modified_time.is_none() {
        builder.last_modified_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn knowledge_base_summary_correct_errors(
    mut builder: crate::types::builders::KnowledgeBaseSummaryBuilder,
) -> crate::types::builders::KnowledgeBaseSummaryBuilder {
    if builder.knowledge_base_id.is_none() {
        builder.knowledge_base_id = Some(Default::default())
    }
    if builder.knowledge_base_arn.is_none() {
        builder.knowledge_base_arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.knowledge_base_type.is_none() {
        builder.knowledge_base_type = "no value was set".parse::<crate::types::KnowledgeBaseType>().ok()
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::KnowledgeBaseStatus>().ok()
    }
    builder
}

pub(crate) fn quick_response_search_result_data_correct_errors(
    mut builder: crate::types::builders::QuickResponseSearchResultDataBuilder,
) -> crate::types::builders::QuickResponseSearchResultDataBuilder {
    if builder.quick_response_arn.is_none() {
        builder.quick_response_arn = Some(Default::default())
    }
    if builder.quick_response_id.is_none() {
        builder.quick_response_id = Some(Default::default())
    }
    if builder.knowledge_base_arn.is_none() {
        builder.knowledge_base_arn = Some(Default::default())
    }
    if builder.knowledge_base_id.is_none() {
        builder.knowledge_base_id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.content_type.is_none() {
        builder.content_type = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::QuickResponseStatus>().ok()
    }
    if builder.contents.is_none() {
        builder.contents = {
            let builder = crate::types::builders::QuickResponseContentsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.created_time.is_none() {
        builder.created_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.last_modified_time.is_none() {
        builder.last_modified_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.is_active.is_none() {
        builder.is_active = Some(Default::default())
    }
    builder
}

pub(crate) fn quick_response_summary_correct_errors(
    mut builder: crate::types::builders::QuickResponseSummaryBuilder,
) -> crate::types::builders::QuickResponseSummaryBuilder {
    if builder.quick_response_arn.is_none() {
        builder.quick_response_arn = Some(Default::default())
    }
    if builder.quick_response_id.is_none() {
        builder.quick_response_id = Some(Default::default())
    }
    if builder.knowledge_base_arn.is_none() {
        builder.knowledge_base_arn = Some(Default::default())
    }
    if builder.knowledge_base_id.is_none() {
        builder.knowledge_base_id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.content_type.is_none() {
        builder.content_type = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::QuickResponseStatus>().ok()
    }
    if builder.created_time.is_none() {
        builder.created_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.last_modified_time.is_none() {
        builder.last_modified_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn recommendation_data_correct_errors(
    mut builder: crate::types::builders::RecommendationDataBuilder,
) -> crate::types::builders::RecommendationDataBuilder {
    if builder.recommendation_id.is_none() {
        builder.recommendation_id = Some(Default::default())
    }
    builder
}

pub(crate) fn recommendation_trigger_correct_errors(
    mut builder: crate::types::builders::RecommendationTriggerBuilder,
) -> crate::types::builders::RecommendationTriggerBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::RecommendationTriggerType>().ok()
    }
    if builder.source.is_none() {
        builder.source = "no value was set".parse::<crate::types::RecommendationSourceType>().ok()
    }
    if builder.data.is_none() {
        builder.data = Some(crate::types::RecommendationTriggerData::Unknown)
    }
    if builder.recommendation_ids.is_none() {
        builder.recommendation_ids = Some(Default::default())
    }
    builder
}

pub(crate) fn result_data_correct_errors(mut builder: crate::types::builders::ResultDataBuilder) -> crate::types::builders::ResultDataBuilder {
    if builder.result_id.is_none() {
        builder.result_id = Some(Default::default())
    }
    builder
}

pub(crate) fn session_summary_correct_errors(
    mut builder: crate::types::builders::SessionSummaryBuilder,
) -> crate::types::builders::SessionSummaryBuilder {
    if builder.session_id.is_none() {
        builder.session_id = Some(Default::default())
    }
    if builder.session_arn.is_none() {
        builder.session_arn = Some(Default::default())
    }
    if builder.assistant_id.is_none() {
        builder.assistant_id = Some(Default::default())
    }
    if builder.assistant_arn.is_none() {
        builder.assistant_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn app_integrations_configuration_correct_errors(
    mut builder: crate::types::builders::AppIntegrationsConfigurationBuilder,
) -> crate::types::builders::AppIntegrationsConfigurationBuilder {
    if builder.app_integration_arn.is_none() {
        builder.app_integration_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn data_summary_correct_errors(mut builder: crate::types::builders::DataSummaryBuilder) -> crate::types::builders::DataSummaryBuilder {
    if builder.reference.is_none() {
        builder.reference = Some(crate::types::DataReference::Unknown)
    }
    if builder.details.is_none() {
        builder.details = Some(crate::types::DataDetails::Unknown)
    }
    builder
}

pub(crate) fn document_correct_errors(mut builder: crate::types::builders::DocumentBuilder) -> crate::types::builders::DocumentBuilder {
    if builder.content_reference.is_none() {
        builder.content_reference = {
            let builder = crate::types::builders::ContentReferenceBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn content_data_details_correct_errors(
    mut builder: crate::types::builders::ContentDataDetailsBuilder,
) -> crate::types::builders::ContentDataDetailsBuilder {
    if builder.text_data.is_none() {
        builder.text_data = {
            let builder = crate::types::builders::TextDataBuilder::default();
            Some(builder.build())
        }
    }
    if builder.ranking_data.is_none() {
        builder.ranking_data = {
            let builder = crate::types::builders::RankingDataBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn generative_data_details_correct_errors(
    mut builder: crate::types::builders::GenerativeDataDetailsBuilder,
) -> crate::types::builders::GenerativeDataDetailsBuilder {
    if builder.completion.is_none() {
        builder.completion = Some(Default::default())
    }
    if builder.references.is_none() {
        builder.references = Some(Default::default())
    }
    if builder.ranking_data.is_none() {
        builder.ranking_data = {
            let builder = crate::types::builders::RankingDataBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn source_content_data_details_correct_errors(
    mut builder: crate::types::builders::SourceContentDataDetailsBuilder,
) -> crate::types::builders::SourceContentDataDetailsBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::SourceContentType>().ok()
    }
    if builder.text_data.is_none() {
        builder.text_data = {
            let builder = crate::types::builders::TextDataBuilder::default();
            Some(builder.build())
        }
    }
    if builder.ranking_data.is_none() {
        builder.ranking_data = {
            let builder = crate::types::builders::RankingDataBuilder::default();
            Some(builder.build())
        }
    }
    builder
}
