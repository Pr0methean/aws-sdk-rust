// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Container for the result of the StartSupportDataExport operation.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartSupportDataExportOutput {
    /// A unique identifier representing a specific request to the StartSupportDataExport operation. This identifier can be used to correlate a request with notifications from the SNS topic.
    pub data_set_request_id: std::option::Option<std::string::String>,
}
impl StartSupportDataExportOutput {
    /// A unique identifier representing a specific request to the StartSupportDataExport operation. This identifier can be used to correlate a request with notifications from the SNS topic.
    pub fn data_set_request_id(&self) -> std::option::Option<&str> {
        self.data_set_request_id.as_deref()
    }
}
impl std::fmt::Debug for StartSupportDataExportOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartSupportDataExportOutput");
        formatter.field("data_set_request_id", &self.data_set_request_id);
        formatter.finish()
    }
}
/// See [`StartSupportDataExportOutput`](crate::output::StartSupportDataExportOutput)
pub mod start_support_data_export_output {

    /// A builder for [`StartSupportDataExportOutput`](crate::output::StartSupportDataExportOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) data_set_request_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// A unique identifier representing a specific request to the StartSupportDataExport operation. This identifier can be used to correlate a request with notifications from the SNS topic.
        pub fn data_set_request_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.data_set_request_id = Some(input.into());
            self
        }
        /// A unique identifier representing a specific request to the StartSupportDataExport operation. This identifier can be used to correlate a request with notifications from the SNS topic.
        pub fn set_data_set_request_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.data_set_request_id = input;
            self
        }
        /// Consumes the builder and constructs a [`StartSupportDataExportOutput`](crate::output::StartSupportDataExportOutput)
        pub fn build(self) -> crate::output::StartSupportDataExportOutput {
            crate::output::StartSupportDataExportOutput {
                data_set_request_id: self.data_set_request_id,
            }
        }
    }
}
impl StartSupportDataExportOutput {
    /// Creates a new builder-style object to manufacture [`StartSupportDataExportOutput`](crate::output::StartSupportDataExportOutput)
    pub fn builder() -> crate::output::start_support_data_export_output::Builder {
        crate::output::start_support_data_export_output::Builder::default()
    }
}

/// Container for the result of the GenerateDataSet operation.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GenerateDataSetOutput {
    /// A unique identifier representing a specific request to the GenerateDataSet operation. This identifier can be used to correlate a request with notifications from the SNS topic.
    pub data_set_request_id: std::option::Option<std::string::String>,
}
impl GenerateDataSetOutput {
    /// A unique identifier representing a specific request to the GenerateDataSet operation. This identifier can be used to correlate a request with notifications from the SNS topic.
    pub fn data_set_request_id(&self) -> std::option::Option<&str> {
        self.data_set_request_id.as_deref()
    }
}
impl std::fmt::Debug for GenerateDataSetOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GenerateDataSetOutput");
        formatter.field("data_set_request_id", &self.data_set_request_id);
        formatter.finish()
    }
}
/// See [`GenerateDataSetOutput`](crate::output::GenerateDataSetOutput)
pub mod generate_data_set_output {

    /// A builder for [`GenerateDataSetOutput`](crate::output::GenerateDataSetOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) data_set_request_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// A unique identifier representing a specific request to the GenerateDataSet operation. This identifier can be used to correlate a request with notifications from the SNS topic.
        pub fn data_set_request_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.data_set_request_id = Some(input.into());
            self
        }
        /// A unique identifier representing a specific request to the GenerateDataSet operation. This identifier can be used to correlate a request with notifications from the SNS topic.
        pub fn set_data_set_request_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.data_set_request_id = input;
            self
        }
        /// Consumes the builder and constructs a [`GenerateDataSetOutput`](crate::output::GenerateDataSetOutput)
        pub fn build(self) -> crate::output::GenerateDataSetOutput {
            crate::output::GenerateDataSetOutput {
                data_set_request_id: self.data_set_request_id,
            }
        }
    }
}
impl GenerateDataSetOutput {
    /// Creates a new builder-style object to manufacture [`GenerateDataSetOutput`](crate::output::GenerateDataSetOutput)
    pub fn builder() -> crate::output::generate_data_set_output::Builder {
        crate::output::generate_data_set_output::Builder::default()
    }
}
