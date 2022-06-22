// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The definition of AWS Cost and Usage Report. You can specify the report name, time unit, report format, compression format, S3 bucket, additional artifacts, and schema elements in the definition. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ReportDefinition {
    /// <p>The name of the report that you want to create. The name must be unique, is case sensitive, and can't include spaces. </p>
    pub report_name: std::option::Option<std::string::String>,
    /// <p>The length of time covered by the report. </p>
    pub time_unit: std::option::Option<crate::model::TimeUnit>,
    /// <p>The format that AWS saves the report in.</p>
    pub format: std::option::Option<crate::model::ReportFormat>,
    /// <p>The compression format that AWS uses for the report.</p>
    pub compression: std::option::Option<crate::model::CompressionFormat>,
    /// <p>A list of strings that indicate additional content that Amazon Web Services includes in the report, such as individual resource IDs. </p>
    pub additional_schema_elements: std::option::Option<std::vec::Vec<crate::model::SchemaElement>>,
    /// <p>The S3 bucket where AWS delivers the report.</p>
    pub s3_bucket: std::option::Option<std::string::String>,
    /// <p>The prefix that AWS adds to the report name when AWS delivers the report. Your prefix can't include spaces.</p>
    pub s3_prefix: std::option::Option<std::string::String>,
    /// <p>The region of the S3 bucket that AWS delivers the report into.</p>
    pub s3_region: std::option::Option<crate::model::AwsRegion>,
    /// <p>A list of manifests that you want Amazon Web Services to create for this report.</p>
    pub additional_artifacts: std::option::Option<std::vec::Vec<crate::model::AdditionalArtifact>>,
    /// <p>Whether you want Amazon Web Services to update your reports after they have been finalized if Amazon Web Services detects charges related to previous months. These charges can include refunds, credits, or support fees.</p>
    pub refresh_closed_reports: std::option::Option<bool>,
    /// <p>Whether you want Amazon Web Services to overwrite the previous version of each report or to deliver the report in addition to the previous versions.</p>
    pub report_versioning: std::option::Option<crate::model::ReportVersioning>,
    /// <p> The Amazon resource name of the billing view. You can get this value by using the billing view service public APIs. </p>
    pub billing_view_arn: std::option::Option<std::string::String>,
}
impl ReportDefinition {
    /// <p>The name of the report that you want to create. The name must be unique, is case sensitive, and can't include spaces. </p>
    pub fn report_name(&self) -> std::option::Option<&str> {
        self.report_name.as_deref()
    }
    /// <p>The length of time covered by the report. </p>
    pub fn time_unit(&self) -> std::option::Option<&crate::model::TimeUnit> {
        self.time_unit.as_ref()
    }
    /// <p>The format that AWS saves the report in.</p>
    pub fn format(&self) -> std::option::Option<&crate::model::ReportFormat> {
        self.format.as_ref()
    }
    /// <p>The compression format that AWS uses for the report.</p>
    pub fn compression(&self) -> std::option::Option<&crate::model::CompressionFormat> {
        self.compression.as_ref()
    }
    /// <p>A list of strings that indicate additional content that Amazon Web Services includes in the report, such as individual resource IDs. </p>
    pub fn additional_schema_elements(
        &self,
    ) -> std::option::Option<&[crate::model::SchemaElement]> {
        self.additional_schema_elements.as_deref()
    }
    /// <p>The S3 bucket where AWS delivers the report.</p>
    pub fn s3_bucket(&self) -> std::option::Option<&str> {
        self.s3_bucket.as_deref()
    }
    /// <p>The prefix that AWS adds to the report name when AWS delivers the report. Your prefix can't include spaces.</p>
    pub fn s3_prefix(&self) -> std::option::Option<&str> {
        self.s3_prefix.as_deref()
    }
    /// <p>The region of the S3 bucket that AWS delivers the report into.</p>
    pub fn s3_region(&self) -> std::option::Option<&crate::model::AwsRegion> {
        self.s3_region.as_ref()
    }
    /// <p>A list of manifests that you want Amazon Web Services to create for this report.</p>
    pub fn additional_artifacts(&self) -> std::option::Option<&[crate::model::AdditionalArtifact]> {
        self.additional_artifacts.as_deref()
    }
    /// <p>Whether you want Amazon Web Services to update your reports after they have been finalized if Amazon Web Services detects charges related to previous months. These charges can include refunds, credits, or support fees.</p>
    pub fn refresh_closed_reports(&self) -> std::option::Option<bool> {
        self.refresh_closed_reports
    }
    /// <p>Whether you want Amazon Web Services to overwrite the previous version of each report or to deliver the report in addition to the previous versions.</p>
    pub fn report_versioning(&self) -> std::option::Option<&crate::model::ReportVersioning> {
        self.report_versioning.as_ref()
    }
    /// <p> The Amazon resource name of the billing view. You can get this value by using the billing view service public APIs. </p>
    pub fn billing_view_arn(&self) -> std::option::Option<&str> {
        self.billing_view_arn.as_deref()
    }
}
impl std::fmt::Debug for ReportDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ReportDefinition");
        formatter.field("report_name", &self.report_name);
        formatter.field("time_unit", &self.time_unit);
        formatter.field("format", &self.format);
        formatter.field("compression", &self.compression);
        formatter.field(
            "additional_schema_elements",
            &self.additional_schema_elements,
        );
        formatter.field("s3_bucket", &self.s3_bucket);
        formatter.field("s3_prefix", &self.s3_prefix);
        formatter.field("s3_region", &self.s3_region);
        formatter.field("additional_artifacts", &self.additional_artifacts);
        formatter.field("refresh_closed_reports", &self.refresh_closed_reports);
        formatter.field("report_versioning", &self.report_versioning);
        formatter.field("billing_view_arn", &self.billing_view_arn);
        formatter.finish()
    }
}
/// See [`ReportDefinition`](crate::model::ReportDefinition)
pub mod report_definition {

    /// A builder for [`ReportDefinition`](crate::model::ReportDefinition)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_name: std::option::Option<std::string::String>,
        pub(crate) time_unit: std::option::Option<crate::model::TimeUnit>,
        pub(crate) format: std::option::Option<crate::model::ReportFormat>,
        pub(crate) compression: std::option::Option<crate::model::CompressionFormat>,
        pub(crate) additional_schema_elements:
            std::option::Option<std::vec::Vec<crate::model::SchemaElement>>,
        pub(crate) s3_bucket: std::option::Option<std::string::String>,
        pub(crate) s3_prefix: std::option::Option<std::string::String>,
        pub(crate) s3_region: std::option::Option<crate::model::AwsRegion>,
        pub(crate) additional_artifacts:
            std::option::Option<std::vec::Vec<crate::model::AdditionalArtifact>>,
        pub(crate) refresh_closed_reports: std::option::Option<bool>,
        pub(crate) report_versioning: std::option::Option<crate::model::ReportVersioning>,
        pub(crate) billing_view_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the report that you want to create. The name must be unique, is case sensitive, and can't include spaces. </p>
        pub fn report_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_name = Some(input.into());
            self
        }
        /// <p>The name of the report that you want to create. The name must be unique, is case sensitive, and can't include spaces. </p>
        pub fn set_report_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.report_name = input;
            self
        }
        /// <p>The length of time covered by the report. </p>
        pub fn time_unit(mut self, input: crate::model::TimeUnit) -> Self {
            self.time_unit = Some(input);
            self
        }
        /// <p>The length of time covered by the report. </p>
        pub fn set_time_unit(mut self, input: std::option::Option<crate::model::TimeUnit>) -> Self {
            self.time_unit = input;
            self
        }
        /// <p>The format that AWS saves the report in.</p>
        pub fn format(mut self, input: crate::model::ReportFormat) -> Self {
            self.format = Some(input);
            self
        }
        /// <p>The format that AWS saves the report in.</p>
        pub fn set_format(
            mut self,
            input: std::option::Option<crate::model::ReportFormat>,
        ) -> Self {
            self.format = input;
            self
        }
        /// <p>The compression format that AWS uses for the report.</p>
        pub fn compression(mut self, input: crate::model::CompressionFormat) -> Self {
            self.compression = Some(input);
            self
        }
        /// <p>The compression format that AWS uses for the report.</p>
        pub fn set_compression(
            mut self,
            input: std::option::Option<crate::model::CompressionFormat>,
        ) -> Self {
            self.compression = input;
            self
        }
        /// Appends an item to `additional_schema_elements`.
        ///
        /// To override the contents of this collection use [`set_additional_schema_elements`](Self::set_additional_schema_elements).
        ///
        /// <p>A list of strings that indicate additional content that Amazon Web Services includes in the report, such as individual resource IDs. </p>
        pub fn additional_schema_elements(mut self, input: crate::model::SchemaElement) -> Self {
            let mut v = self.additional_schema_elements.unwrap_or_default();
            v.push(input);
            self.additional_schema_elements = Some(v);
            self
        }
        /// <p>A list of strings that indicate additional content that Amazon Web Services includes in the report, such as individual resource IDs. </p>
        pub fn set_additional_schema_elements(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SchemaElement>>,
        ) -> Self {
            self.additional_schema_elements = input;
            self
        }
        /// <p>The S3 bucket where AWS delivers the report.</p>
        pub fn s3_bucket(mut self, input: impl Into<std::string::String>) -> Self {
            self.s3_bucket = Some(input.into());
            self
        }
        /// <p>The S3 bucket where AWS delivers the report.</p>
        pub fn set_s3_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.s3_bucket = input;
            self
        }
        /// <p>The prefix that AWS adds to the report name when AWS delivers the report. Your prefix can't include spaces.</p>
        pub fn s3_prefix(mut self, input: impl Into<std::string::String>) -> Self {
            self.s3_prefix = Some(input.into());
            self
        }
        /// <p>The prefix that AWS adds to the report name when AWS delivers the report. Your prefix can't include spaces.</p>
        pub fn set_s3_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.s3_prefix = input;
            self
        }
        /// <p>The region of the S3 bucket that AWS delivers the report into.</p>
        pub fn s3_region(mut self, input: crate::model::AwsRegion) -> Self {
            self.s3_region = Some(input);
            self
        }
        /// <p>The region of the S3 bucket that AWS delivers the report into.</p>
        pub fn set_s3_region(
            mut self,
            input: std::option::Option<crate::model::AwsRegion>,
        ) -> Self {
            self.s3_region = input;
            self
        }
        /// Appends an item to `additional_artifacts`.
        ///
        /// To override the contents of this collection use [`set_additional_artifacts`](Self::set_additional_artifacts).
        ///
        /// <p>A list of manifests that you want Amazon Web Services to create for this report.</p>
        pub fn additional_artifacts(mut self, input: crate::model::AdditionalArtifact) -> Self {
            let mut v = self.additional_artifacts.unwrap_or_default();
            v.push(input);
            self.additional_artifacts = Some(v);
            self
        }
        /// <p>A list of manifests that you want Amazon Web Services to create for this report.</p>
        pub fn set_additional_artifacts(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::AdditionalArtifact>>,
        ) -> Self {
            self.additional_artifacts = input;
            self
        }
        /// <p>Whether you want Amazon Web Services to update your reports after they have been finalized if Amazon Web Services detects charges related to previous months. These charges can include refunds, credits, or support fees.</p>
        pub fn refresh_closed_reports(mut self, input: bool) -> Self {
            self.refresh_closed_reports = Some(input);
            self
        }
        /// <p>Whether you want Amazon Web Services to update your reports after they have been finalized if Amazon Web Services detects charges related to previous months. These charges can include refunds, credits, or support fees.</p>
        pub fn set_refresh_closed_reports(mut self, input: std::option::Option<bool>) -> Self {
            self.refresh_closed_reports = input;
            self
        }
        /// <p>Whether you want Amazon Web Services to overwrite the previous version of each report or to deliver the report in addition to the previous versions.</p>
        pub fn report_versioning(mut self, input: crate::model::ReportVersioning) -> Self {
            self.report_versioning = Some(input);
            self
        }
        /// <p>Whether you want Amazon Web Services to overwrite the previous version of each report or to deliver the report in addition to the previous versions.</p>
        pub fn set_report_versioning(
            mut self,
            input: std::option::Option<crate::model::ReportVersioning>,
        ) -> Self {
            self.report_versioning = input;
            self
        }
        /// <p> The Amazon resource name of the billing view. You can get this value by using the billing view service public APIs. </p>
        pub fn billing_view_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.billing_view_arn = Some(input.into());
            self
        }
        /// <p> The Amazon resource name of the billing view. You can get this value by using the billing view service public APIs. </p>
        pub fn set_billing_view_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.billing_view_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`ReportDefinition`](crate::model::ReportDefinition)
        pub fn build(self) -> crate::model::ReportDefinition {
            crate::model::ReportDefinition {
                report_name: self.report_name,
                time_unit: self.time_unit,
                format: self.format,
                compression: self.compression,
                additional_schema_elements: self.additional_schema_elements,
                s3_bucket: self.s3_bucket,
                s3_prefix: self.s3_prefix,
                s3_region: self.s3_region,
                additional_artifacts: self.additional_artifacts,
                refresh_closed_reports: self.refresh_closed_reports,
                report_versioning: self.report_versioning,
                billing_view_arn: self.billing_view_arn,
            }
        }
    }
}
impl ReportDefinition {
    /// Creates a new builder-style object to manufacture [`ReportDefinition`](crate::model::ReportDefinition)
    pub fn builder() -> crate::model::report_definition::Builder {
        crate::model::report_definition::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ReportVersioning {
    #[allow(missing_docs)] // documentation missing in model
    CreateNewReport,
    #[allow(missing_docs)] // documentation missing in model
    OverwriteReport,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ReportVersioning {
    fn from(s: &str) -> Self {
        match s {
            "CREATE_NEW_REPORT" => ReportVersioning::CreateNewReport,
            "OVERWRITE_REPORT" => ReportVersioning::OverwriteReport,
            other => ReportVersioning::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ReportVersioning {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ReportVersioning::from(s))
    }
}
impl ReportVersioning {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ReportVersioning::CreateNewReport => "CREATE_NEW_REPORT",
            ReportVersioning::OverwriteReport => "OVERWRITE_REPORT",
            ReportVersioning::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["CREATE_NEW_REPORT", "OVERWRITE_REPORT"]
    }
}
impl AsRef<str> for ReportVersioning {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The types of manifest that you want AWS to create for this report.</p>
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum AdditionalArtifact {
    #[allow(missing_docs)] // documentation missing in model
    Athena,
    #[allow(missing_docs)] // documentation missing in model
    Quicksight,
    #[allow(missing_docs)] // documentation missing in model
    Redshift,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for AdditionalArtifact {
    fn from(s: &str) -> Self {
        match s {
            "ATHENA" => AdditionalArtifact::Athena,
            "QUICKSIGHT" => AdditionalArtifact::Quicksight,
            "REDSHIFT" => AdditionalArtifact::Redshift,
            other => AdditionalArtifact::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for AdditionalArtifact {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(AdditionalArtifact::from(s))
    }
}
impl AdditionalArtifact {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            AdditionalArtifact::Athena => "ATHENA",
            AdditionalArtifact::Quicksight => "QUICKSIGHT",
            AdditionalArtifact::Redshift => "REDSHIFT",
            AdditionalArtifact::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["ATHENA", "QUICKSIGHT", "REDSHIFT"]
    }
}
impl AsRef<str> for AdditionalArtifact {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The region of the S3 bucket that AWS delivers the report into.</p>
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum AwsRegion {
    #[allow(missing_docs)] // documentation missing in model
    CapeTown,
    #[allow(missing_docs)] // documentation missing in model
    HongKong,
    #[allow(missing_docs)] // documentation missing in model
    Tokyo,
    #[allow(missing_docs)] // documentation missing in model
    Seoul,
    #[allow(missing_docs)] // documentation missing in model
    Osaka,
    #[allow(missing_docs)] // documentation missing in model
    Mumbai,
    #[allow(missing_docs)] // documentation missing in model
    Singapore,
    #[allow(missing_docs)] // documentation missing in model
    Sydney,
    #[allow(missing_docs)] // documentation missing in model
    CanadaCentral,
    #[allow(missing_docs)] // documentation missing in model
    Beijing,
    #[allow(missing_docs)] // documentation missing in model
    Ningxia,
    #[allow(missing_docs)] // documentation missing in model
    Frankfurt,
    #[allow(missing_docs)] // documentation missing in model
    Stockholm,
    #[allow(missing_docs)] // documentation missing in model
    Milano,
    #[allow(missing_docs)] // documentation missing in model
    Ireland,
    #[allow(missing_docs)] // documentation missing in model
    London,
    #[allow(missing_docs)] // documentation missing in model
    Paris,
    #[allow(missing_docs)] // documentation missing in model
    Bahrain,
    #[allow(missing_docs)] // documentation missing in model
    SaoPaulo,
    #[allow(missing_docs)] // documentation missing in model
    UsStandard,
    #[allow(missing_docs)] // documentation missing in model
    Ohio,
    #[allow(missing_docs)] // documentation missing in model
    NorthernCalifornia,
    #[allow(missing_docs)] // documentation missing in model
    Oregon,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for AwsRegion {
    fn from(s: &str) -> Self {
        match s {
            "af-south-1" => AwsRegion::CapeTown,
            "ap-east-1" => AwsRegion::HongKong,
            "ap-northeast-1" => AwsRegion::Tokyo,
            "ap-northeast-2" => AwsRegion::Seoul,
            "ap-northeast-3" => AwsRegion::Osaka,
            "ap-south-1" => AwsRegion::Mumbai,
            "ap-southeast-1" => AwsRegion::Singapore,
            "ap-southeast-2" => AwsRegion::Sydney,
            "ca-central-1" => AwsRegion::CanadaCentral,
            "cn-north-1" => AwsRegion::Beijing,
            "cn-northwest-1" => AwsRegion::Ningxia,
            "eu-central-1" => AwsRegion::Frankfurt,
            "eu-north-1" => AwsRegion::Stockholm,
            "eu-south-1" => AwsRegion::Milano,
            "eu-west-1" => AwsRegion::Ireland,
            "eu-west-2" => AwsRegion::London,
            "eu-west-3" => AwsRegion::Paris,
            "me-south-1" => AwsRegion::Bahrain,
            "sa-east-1" => AwsRegion::SaoPaulo,
            "us-east-1" => AwsRegion::UsStandard,
            "us-east-2" => AwsRegion::Ohio,
            "us-west-1" => AwsRegion::NorthernCalifornia,
            "us-west-2" => AwsRegion::Oregon,
            other => AwsRegion::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for AwsRegion {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(AwsRegion::from(s))
    }
}
impl AwsRegion {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            AwsRegion::CapeTown => "af-south-1",
            AwsRegion::HongKong => "ap-east-1",
            AwsRegion::Tokyo => "ap-northeast-1",
            AwsRegion::Seoul => "ap-northeast-2",
            AwsRegion::Osaka => "ap-northeast-3",
            AwsRegion::Mumbai => "ap-south-1",
            AwsRegion::Singapore => "ap-southeast-1",
            AwsRegion::Sydney => "ap-southeast-2",
            AwsRegion::CanadaCentral => "ca-central-1",
            AwsRegion::Beijing => "cn-north-1",
            AwsRegion::Ningxia => "cn-northwest-1",
            AwsRegion::Frankfurt => "eu-central-1",
            AwsRegion::Stockholm => "eu-north-1",
            AwsRegion::Milano => "eu-south-1",
            AwsRegion::Ireland => "eu-west-1",
            AwsRegion::London => "eu-west-2",
            AwsRegion::Paris => "eu-west-3",
            AwsRegion::Bahrain => "me-south-1",
            AwsRegion::SaoPaulo => "sa-east-1",
            AwsRegion::UsStandard => "us-east-1",
            AwsRegion::Ohio => "us-east-2",
            AwsRegion::NorthernCalifornia => "us-west-1",
            AwsRegion::Oregon => "us-west-2",
            AwsRegion::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "af-south-1",
            "ap-east-1",
            "ap-northeast-1",
            "ap-northeast-2",
            "ap-northeast-3",
            "ap-south-1",
            "ap-southeast-1",
            "ap-southeast-2",
            "ca-central-1",
            "cn-north-1",
            "cn-northwest-1",
            "eu-central-1",
            "eu-north-1",
            "eu-south-1",
            "eu-west-1",
            "eu-west-2",
            "eu-west-3",
            "me-south-1",
            "sa-east-1",
            "us-east-1",
            "us-east-2",
            "us-west-1",
            "us-west-2",
        ]
    }
}
impl AsRef<str> for AwsRegion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Whether or not AWS includes resource IDs in the report. </p>
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum SchemaElement {
    #[allow(missing_docs)] // documentation missing in model
    Resources,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for SchemaElement {
    fn from(s: &str) -> Self {
        match s {
            "RESOURCES" => SchemaElement::Resources,
            other => SchemaElement::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for SchemaElement {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SchemaElement::from(s))
    }
}
impl SchemaElement {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            SchemaElement::Resources => "RESOURCES",
            SchemaElement::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["RESOURCES"]
    }
}
impl AsRef<str> for SchemaElement {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The compression format that AWS uses for the report.</p>
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum CompressionFormat {
    #[allow(missing_docs)] // documentation missing in model
    Gzip,
    #[allow(missing_docs)] // documentation missing in model
    Parquet,
    #[allow(missing_docs)] // documentation missing in model
    Zip,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for CompressionFormat {
    fn from(s: &str) -> Self {
        match s {
            "GZIP" => CompressionFormat::Gzip,
            "Parquet" => CompressionFormat::Parquet,
            "ZIP" => CompressionFormat::Zip,
            other => CompressionFormat::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for CompressionFormat {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(CompressionFormat::from(s))
    }
}
impl CompressionFormat {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            CompressionFormat::Gzip => "GZIP",
            CompressionFormat::Parquet => "Parquet",
            CompressionFormat::Zip => "ZIP",
            CompressionFormat::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["GZIP", "Parquet", "ZIP"]
    }
}
impl AsRef<str> for CompressionFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The format that AWS saves the report in.</p>
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ReportFormat {
    #[allow(missing_docs)] // documentation missing in model
    Parquet,
    #[allow(missing_docs)] // documentation missing in model
    Csv,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ReportFormat {
    fn from(s: &str) -> Self {
        match s {
            "Parquet" => ReportFormat::Parquet,
            "textORcsv" => ReportFormat::Csv,
            other => ReportFormat::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ReportFormat {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ReportFormat::from(s))
    }
}
impl ReportFormat {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ReportFormat::Parquet => "Parquet",
            ReportFormat::Csv => "textORcsv",
            ReportFormat::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["Parquet", "textORcsv"]
    }
}
impl AsRef<str> for ReportFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The length of time covered by the report. </p>
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum TimeUnit {
    #[allow(missing_docs)] // documentation missing in model
    Daily,
    #[allow(missing_docs)] // documentation missing in model
    Hourly,
    #[allow(missing_docs)] // documentation missing in model
    Monthly,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for TimeUnit {
    fn from(s: &str) -> Self {
        match s {
            "DAILY" => TimeUnit::Daily,
            "HOURLY" => TimeUnit::Hourly,
            "MONTHLY" => TimeUnit::Monthly,
            other => TimeUnit::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for TimeUnit {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(TimeUnit::from(s))
    }
}
impl TimeUnit {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            TimeUnit::Daily => "DAILY",
            TimeUnit::Hourly => "HOURLY",
            TimeUnit::Monthly => "MONTHLY",
            TimeUnit::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["DAILY", "HOURLY", "MONTHLY"]
    }
}
impl AsRef<str> for TimeUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
