// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSequenceStore`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::set_name):<br>required: **true**<br><p>A name for the store.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::set_description):<br>required: **false**<br><p>A description for the store.</p><br>
    ///   - [`sse_config(SseConfig)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::sse_config) / [`set_sse_config(Option<SseConfig>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::set_sse_config):<br>required: **false**<br><p>Server-side encryption (SSE) settings for the store.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::set_tags):<br>required: **false**<br><p>Tags for the store.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::set_client_token):<br>required: **false**<br><p>To ensure that requests don't run multiple times, specify a unique token for each request.</p><br>
    ///   - [`fallback_location(impl Into<String>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::fallback_location) / [`set_fallback_location(Option<String>)`](crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::set_fallback_location):<br>required: **false**<br><p>An S3 location that is used to store files that have failed a direct upload.</p><br>
    /// - On success, responds with [`CreateSequenceStoreOutput`](crate::operation::create_sequence_store::CreateSequenceStoreOutput) with field(s):
    ///   - [`id(String)`](crate::operation::create_sequence_store::CreateSequenceStoreOutput::id): <p>The store's ID.</p>
    ///   - [`arn(String)`](crate::operation::create_sequence_store::CreateSequenceStoreOutput::arn): <p>The store's ARN.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_sequence_store::CreateSequenceStoreOutput::name): <p>The store's name.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_sequence_store::CreateSequenceStoreOutput::description): <p>The store's description.</p>
    ///   - [`sse_config(Option<SseConfig>)`](crate::operation::create_sequence_store::CreateSequenceStoreOutput::sse_config): <p>The store's SSE settings.</p>
    ///   - [`creation_time(DateTime)`](crate::operation::create_sequence_store::CreateSequenceStoreOutput::creation_time): <p>When the store was created.</p>
    ///   - [`fallback_location(Option<String>)`](crate::operation::create_sequence_store::CreateSequenceStoreOutput::fallback_location): <p>An S3 location that is used to store files that have failed a direct upload.</p>
    /// - On failure, responds with [`SdkError<CreateSequenceStoreError>`](crate::operation::create_sequence_store::CreateSequenceStoreError)
    pub fn create_sequence_store(&self) -> crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder {
        crate::operation::create_sequence_store::builders::CreateSequenceStoreFluentBuilder::new(self.handle.clone())
    }
}
