// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListAvailableResourceMetricsOutput {
    /// <p>An array of metrics available to query. Each array element contains the full name, description, and unit of the metric. </p>
    pub metrics: std::option::Option<std::vec::Vec<crate::model::ResponseResourceMetric>>,
    /// <p>A pagination token that indicates the response didn’t return all available records because <code>MaxRecords</code> was specified in the previous request. To get the remaining records, specify <code>NextToken</code> in a separate request with this value. </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListAvailableResourceMetricsOutput {
    /// <p>An array of metrics available to query. Each array element contains the full name, description, and unit of the metric. </p>
    pub fn metrics(&self) -> std::option::Option<&[crate::model::ResponseResourceMetric]> {
        self.metrics.as_deref()
    }
    /// <p>A pagination token that indicates the response didn’t return all available records because <code>MaxRecords</code> was specified in the previous request. To get the remaining records, specify <code>NextToken</code> in a separate request with this value. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListAvailableResourceMetricsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListAvailableResourceMetricsOutput");
        formatter.field("metrics", &self.metrics);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListAvailableResourceMetricsOutput`](crate::output::ListAvailableResourceMetricsOutput)
pub mod list_available_resource_metrics_output {

    /// A builder for [`ListAvailableResourceMetricsOutput`](crate::output::ListAvailableResourceMetricsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) metrics:
            std::option::Option<std::vec::Vec<crate::model::ResponseResourceMetric>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `metrics`.
        ///
        /// To override the contents of this collection use [`set_metrics`](Self::set_metrics).
        ///
        /// <p>An array of metrics available to query. Each array element contains the full name, description, and unit of the metric. </p>
        pub fn metrics(mut self, input: crate::model::ResponseResourceMetric) -> Self {
            let mut v = self.metrics.unwrap_or_default();
            v.push(input);
            self.metrics = Some(v);
            self
        }
        /// <p>An array of metrics available to query. Each array element contains the full name, description, and unit of the metric. </p>
        pub fn set_metrics(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ResponseResourceMetric>>,
        ) -> Self {
            self.metrics = input;
            self
        }
        /// <p>A pagination token that indicates the response didn’t return all available records because <code>MaxRecords</code> was specified in the previous request. To get the remaining records, specify <code>NextToken</code> in a separate request with this value. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A pagination token that indicates the response didn’t return all available records because <code>MaxRecords</code> was specified in the previous request. To get the remaining records, specify <code>NextToken</code> in a separate request with this value. </p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAvailableResourceMetricsOutput`](crate::output::ListAvailableResourceMetricsOutput)
        pub fn build(self) -> crate::output::ListAvailableResourceMetricsOutput {
            crate::output::ListAvailableResourceMetricsOutput {
                metrics: self.metrics,
                next_token: self.next_token,
            }
        }
    }
}
impl ListAvailableResourceMetricsOutput {
    /// Creates a new builder-style object to manufacture [`ListAvailableResourceMetricsOutput`](crate::output::ListAvailableResourceMetricsOutput)
    pub fn builder() -> crate::output::list_available_resource_metrics_output::Builder {
        crate::output::list_available_resource_metrics_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListAvailableResourceDimensionsOutput {
    /// <p>The dimension information returned for requested metric types.</p>
    pub metric_dimensions: std::option::Option<std::vec::Vec<crate::model::MetricDimensionGroups>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListAvailableResourceDimensionsOutput {
    /// <p>The dimension information returned for requested metric types.</p>
    pub fn metric_dimensions(&self) -> std::option::Option<&[crate::model::MetricDimensionGroups]> {
        self.metric_dimensions.as_deref()
    }
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListAvailableResourceDimensionsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListAvailableResourceDimensionsOutput");
        formatter.field("metric_dimensions", &self.metric_dimensions);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListAvailableResourceDimensionsOutput`](crate::output::ListAvailableResourceDimensionsOutput)
pub mod list_available_resource_dimensions_output {

    /// A builder for [`ListAvailableResourceDimensionsOutput`](crate::output::ListAvailableResourceDimensionsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) metric_dimensions:
            std::option::Option<std::vec::Vec<crate::model::MetricDimensionGroups>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `metric_dimensions`.
        ///
        /// To override the contents of this collection use [`set_metric_dimensions`](Self::set_metric_dimensions).
        ///
        /// <p>The dimension information returned for requested metric types.</p>
        pub fn metric_dimensions(mut self, input: crate::model::MetricDimensionGroups) -> Self {
            let mut v = self.metric_dimensions.unwrap_or_default();
            v.push(input);
            self.metric_dimensions = Some(v);
            self
        }
        /// <p>The dimension information returned for requested metric types.</p>
        pub fn set_metric_dimensions(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::MetricDimensionGroups>>,
        ) -> Self {
            self.metric_dimensions = input;
            self
        }
        /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAvailableResourceDimensionsOutput`](crate::output::ListAvailableResourceDimensionsOutput)
        pub fn build(self) -> crate::output::ListAvailableResourceDimensionsOutput {
            crate::output::ListAvailableResourceDimensionsOutput {
                metric_dimensions: self.metric_dimensions,
                next_token: self.next_token,
            }
        }
    }
}
impl ListAvailableResourceDimensionsOutput {
    /// Creates a new builder-style object to manufacture [`ListAvailableResourceDimensionsOutput`](crate::output::ListAvailableResourceDimensionsOutput)
    pub fn builder() -> crate::output::list_available_resource_dimensions_output::Builder {
        crate::output::list_available_resource_dimensions_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetResourceMetricsOutput {
    /// <p>The start time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>.</p>
    pub aligned_start_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The end time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>.</p>
    pub aligned_end_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. Performance Insights gathers metrics from this data source. In the console, the identifier is shown as <i>ResourceID</i>. When you call <code>DescribeDBInstances</code>, the identifier is returned as <code>DbiResourceId</code>.</p>
    pub identifier: std::option::Option<std::string::String>,
    /// <p>An array of metric results, where each array element contains all of the data points for a particular dimension.</p>
    pub metric_list: std::option::Option<std::vec::Vec<crate::model::MetricKeyDataPoints>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>. </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl GetResourceMetricsOutput {
    /// <p>The start time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>.</p>
    pub fn aligned_start_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.aligned_start_time.as_ref()
    }
    /// <p>The end time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>.</p>
    pub fn aligned_end_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.aligned_end_time.as_ref()
    }
    /// <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. Performance Insights gathers metrics from this data source. In the console, the identifier is shown as <i>ResourceID</i>. When you call <code>DescribeDBInstances</code>, the identifier is returned as <code>DbiResourceId</code>.</p>
    pub fn identifier(&self) -> std::option::Option<&str> {
        self.identifier.as_deref()
    }
    /// <p>An array of metric results, where each array element contains all of the data points for a particular dimension.</p>
    pub fn metric_list(&self) -> std::option::Option<&[crate::model::MetricKeyDataPoints]> {
        self.metric_list.as_deref()
    }
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for GetResourceMetricsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetResourceMetricsOutput");
        formatter.field("aligned_start_time", &self.aligned_start_time);
        formatter.field("aligned_end_time", &self.aligned_end_time);
        formatter.field("identifier", &self.identifier);
        formatter.field("metric_list", &self.metric_list);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`GetResourceMetricsOutput`](crate::output::GetResourceMetricsOutput)
pub mod get_resource_metrics_output {

    /// A builder for [`GetResourceMetricsOutput`](crate::output::GetResourceMetricsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) aligned_start_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) aligned_end_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) identifier: std::option::Option<std::string::String>,
        pub(crate) metric_list:
            std::option::Option<std::vec::Vec<crate::model::MetricKeyDataPoints>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The start time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>.</p>
        pub fn aligned_start_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.aligned_start_time = Some(input);
            self
        }
        /// <p>The start time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>.</p>
        pub fn set_aligned_start_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.aligned_start_time = input;
            self
        }
        /// <p>The end time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>.</p>
        pub fn aligned_end_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.aligned_end_time = Some(input);
            self
        }
        /// <p>The end time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>.</p>
        pub fn set_aligned_end_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.aligned_end_time = input;
            self
        }
        /// <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. Performance Insights gathers metrics from this data source. In the console, the identifier is shown as <i>ResourceID</i>. When you call <code>DescribeDBInstances</code>, the identifier is returned as <code>DbiResourceId</code>.</p>
        pub fn identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.identifier = Some(input.into());
            self
        }
        /// <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. Performance Insights gathers metrics from this data source. In the console, the identifier is shown as <i>ResourceID</i>. When you call <code>DescribeDBInstances</code>, the identifier is returned as <code>DbiResourceId</code>.</p>
        pub fn set_identifier(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.identifier = input;
            self
        }
        /// Appends an item to `metric_list`.
        ///
        /// To override the contents of this collection use [`set_metric_list`](Self::set_metric_list).
        ///
        /// <p>An array of metric results, where each array element contains all of the data points for a particular dimension.</p>
        pub fn metric_list(mut self, input: crate::model::MetricKeyDataPoints) -> Self {
            let mut v = self.metric_list.unwrap_or_default();
            v.push(input);
            self.metric_list = Some(v);
            self
        }
        /// <p>An array of metric results, where each array element contains all of the data points for a particular dimension.</p>
        pub fn set_metric_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::MetricKeyDataPoints>>,
        ) -> Self {
            self.metric_list = input;
            self
        }
        /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>. </p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`GetResourceMetricsOutput`](crate::output::GetResourceMetricsOutput)
        pub fn build(self) -> crate::output::GetResourceMetricsOutput {
            crate::output::GetResourceMetricsOutput {
                aligned_start_time: self.aligned_start_time,
                aligned_end_time: self.aligned_end_time,
                identifier: self.identifier,
                metric_list: self.metric_list,
                next_token: self.next_token,
            }
        }
    }
}
impl GetResourceMetricsOutput {
    /// Creates a new builder-style object to manufacture [`GetResourceMetricsOutput`](crate::output::GetResourceMetricsOutput)
    pub fn builder() -> crate::output::get_resource_metrics_output::Builder {
        crate::output::get_resource_metrics_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetResourceMetadataOutput {
    /// <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. Performance Insights gathers metrics from this data source. To use a DB instance as a data source, specify its <code>DbiResourceId</code> value. For example, specify <code>db-ABCDEFGHIJKLMNOPQRSTU1VW2X</code>. </p>
    pub identifier: std::option::Option<std::string::String>,
    /// <p>The metadata for different features. For example, the metadata might indicate that a feature is turned on or off on a specific DB instance.</p>
    pub features: std::option::Option<
        std::collections::HashMap<std::string::String, crate::model::FeatureMetadata>,
    >,
}
impl GetResourceMetadataOutput {
    /// <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. Performance Insights gathers metrics from this data source. To use a DB instance as a data source, specify its <code>DbiResourceId</code> value. For example, specify <code>db-ABCDEFGHIJKLMNOPQRSTU1VW2X</code>. </p>
    pub fn identifier(&self) -> std::option::Option<&str> {
        self.identifier.as_deref()
    }
    /// <p>The metadata for different features. For example, the metadata might indicate that a feature is turned on or off on a specific DB instance.</p>
    pub fn features(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::model::FeatureMetadata>,
    > {
        self.features.as_ref()
    }
}
impl std::fmt::Debug for GetResourceMetadataOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetResourceMetadataOutput");
        formatter.field("identifier", &self.identifier);
        formatter.field("features", &self.features);
        formatter.finish()
    }
}
/// See [`GetResourceMetadataOutput`](crate::output::GetResourceMetadataOutput)
pub mod get_resource_metadata_output {

    /// A builder for [`GetResourceMetadataOutput`](crate::output::GetResourceMetadataOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) identifier: std::option::Option<std::string::String>,
        pub(crate) features: std::option::Option<
            std::collections::HashMap<std::string::String, crate::model::FeatureMetadata>,
        >,
    }
    impl Builder {
        /// <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. Performance Insights gathers metrics from this data source. To use a DB instance as a data source, specify its <code>DbiResourceId</code> value. For example, specify <code>db-ABCDEFGHIJKLMNOPQRSTU1VW2X</code>. </p>
        pub fn identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.identifier = Some(input.into());
            self
        }
        /// <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. Performance Insights gathers metrics from this data source. To use a DB instance as a data source, specify its <code>DbiResourceId</code> value. For example, specify <code>db-ABCDEFGHIJKLMNOPQRSTU1VW2X</code>. </p>
        pub fn set_identifier(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.identifier = input;
            self
        }
        /// Adds a key-value pair to `features`.
        ///
        /// To override the contents of this collection use [`set_features`](Self::set_features).
        ///
        /// <p>The metadata for different features. For example, the metadata might indicate that a feature is turned on or off on a specific DB instance.</p>
        pub fn features(
            mut self,
            k: impl Into<std::string::String>,
            v: crate::model::FeatureMetadata,
        ) -> Self {
            let mut hash_map = self.features.unwrap_or_default();
            hash_map.insert(k.into(), v);
            self.features = Some(hash_map);
            self
        }
        /// <p>The metadata for different features. For example, the metadata might indicate that a feature is turned on or off on a specific DB instance.</p>
        pub fn set_features(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, crate::model::FeatureMetadata>,
            >,
        ) -> Self {
            self.features = input;
            self
        }
        /// Consumes the builder and constructs a [`GetResourceMetadataOutput`](crate::output::GetResourceMetadataOutput)
        pub fn build(self) -> crate::output::GetResourceMetadataOutput {
            crate::output::GetResourceMetadataOutput {
                identifier: self.identifier,
                features: self.features,
            }
        }
    }
}
impl GetResourceMetadataOutput {
    /// Creates a new builder-style object to manufacture [`GetResourceMetadataOutput`](crate::output::GetResourceMetadataOutput)
    pub fn builder() -> crate::output::get_resource_metadata_output::Builder {
        crate::output::get_resource_metadata_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetDimensionKeyDetailsOutput {
    /// <p>The details for the requested dimensions.</p>
    pub dimensions: std::option::Option<std::vec::Vec<crate::model::DimensionKeyDetail>>,
}
impl GetDimensionKeyDetailsOutput {
    /// <p>The details for the requested dimensions.</p>
    pub fn dimensions(&self) -> std::option::Option<&[crate::model::DimensionKeyDetail]> {
        self.dimensions.as_deref()
    }
}
impl std::fmt::Debug for GetDimensionKeyDetailsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetDimensionKeyDetailsOutput");
        formatter.field("dimensions", &self.dimensions);
        formatter.finish()
    }
}
/// See [`GetDimensionKeyDetailsOutput`](crate::output::GetDimensionKeyDetailsOutput)
pub mod get_dimension_key_details_output {

    /// A builder for [`GetDimensionKeyDetailsOutput`](crate::output::GetDimensionKeyDetailsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) dimensions: std::option::Option<std::vec::Vec<crate::model::DimensionKeyDetail>>,
    }
    impl Builder {
        /// Appends an item to `dimensions`.
        ///
        /// To override the contents of this collection use [`set_dimensions`](Self::set_dimensions).
        ///
        /// <p>The details for the requested dimensions.</p>
        pub fn dimensions(mut self, input: crate::model::DimensionKeyDetail) -> Self {
            let mut v = self.dimensions.unwrap_or_default();
            v.push(input);
            self.dimensions = Some(v);
            self
        }
        /// <p>The details for the requested dimensions.</p>
        pub fn set_dimensions(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::DimensionKeyDetail>>,
        ) -> Self {
            self.dimensions = input;
            self
        }
        /// Consumes the builder and constructs a [`GetDimensionKeyDetailsOutput`](crate::output::GetDimensionKeyDetailsOutput)
        pub fn build(self) -> crate::output::GetDimensionKeyDetailsOutput {
            crate::output::GetDimensionKeyDetailsOutput {
                dimensions: self.dimensions,
            }
        }
    }
}
impl GetDimensionKeyDetailsOutput {
    /// Creates a new builder-style object to manufacture [`GetDimensionKeyDetailsOutput`](crate::output::GetDimensionKeyDetailsOutput)
    pub fn builder() -> crate::output::get_dimension_key_details_output::Builder {
        crate::output::get_dimension_key_details_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeDimensionKeysOutput {
    /// <p>The start time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>. </p>
    pub aligned_start_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The end time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>. </p>
    pub aligned_end_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>If <code>PartitionBy</code> was present in the request, <code>PartitionKeys</code> contains the breakdown of dimension keys by the specified partitions. </p>
    pub partition_keys: std::option::Option<std::vec::Vec<crate::model::ResponsePartitionKey>>,
    /// <p>The dimension keys that were requested.</p>
    pub keys: std::option::Option<std::vec::Vec<crate::model::DimensionKeyDescription>>,
    /// <p>A pagination token that indicates the response didn’t return all available records because <code>MaxRecords</code> was specified in the previous request. To get the remaining records, specify <code>NextToken</code> in a separate request with this value. </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeDimensionKeysOutput {
    /// <p>The start time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>. </p>
    pub fn aligned_start_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.aligned_start_time.as_ref()
    }
    /// <p>The end time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>. </p>
    pub fn aligned_end_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.aligned_end_time.as_ref()
    }
    /// <p>If <code>PartitionBy</code> was present in the request, <code>PartitionKeys</code> contains the breakdown of dimension keys by the specified partitions. </p>
    pub fn partition_keys(&self) -> std::option::Option<&[crate::model::ResponsePartitionKey]> {
        self.partition_keys.as_deref()
    }
    /// <p>The dimension keys that were requested.</p>
    pub fn keys(&self) -> std::option::Option<&[crate::model::DimensionKeyDescription]> {
        self.keys.as_deref()
    }
    /// <p>A pagination token that indicates the response didn’t return all available records because <code>MaxRecords</code> was specified in the previous request. To get the remaining records, specify <code>NextToken</code> in a separate request with this value. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for DescribeDimensionKeysOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeDimensionKeysOutput");
        formatter.field("aligned_start_time", &self.aligned_start_time);
        formatter.field("aligned_end_time", &self.aligned_end_time);
        formatter.field("partition_keys", &self.partition_keys);
        formatter.field("keys", &self.keys);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeDimensionKeysOutput`](crate::output::DescribeDimensionKeysOutput)
pub mod describe_dimension_keys_output {

    /// A builder for [`DescribeDimensionKeysOutput`](crate::output::DescribeDimensionKeysOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) aligned_start_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) aligned_end_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) partition_keys:
            std::option::Option<std::vec::Vec<crate::model::ResponsePartitionKey>>,
        pub(crate) keys: std::option::Option<std::vec::Vec<crate::model::DimensionKeyDescription>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The start time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>. </p>
        pub fn aligned_start_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.aligned_start_time = Some(input);
            self
        }
        /// <p>The start time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>. </p>
        pub fn set_aligned_start_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.aligned_start_time = input;
            self
        }
        /// <p>The end time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>. </p>
        pub fn aligned_end_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.aligned_end_time = Some(input);
            self
        }
        /// <p>The end time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>. </p>
        pub fn set_aligned_end_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.aligned_end_time = input;
            self
        }
        /// Appends an item to `partition_keys`.
        ///
        /// To override the contents of this collection use [`set_partition_keys`](Self::set_partition_keys).
        ///
        /// <p>If <code>PartitionBy</code> was present in the request, <code>PartitionKeys</code> contains the breakdown of dimension keys by the specified partitions. </p>
        pub fn partition_keys(mut self, input: crate::model::ResponsePartitionKey) -> Self {
            let mut v = self.partition_keys.unwrap_or_default();
            v.push(input);
            self.partition_keys = Some(v);
            self
        }
        /// <p>If <code>PartitionBy</code> was present in the request, <code>PartitionKeys</code> contains the breakdown of dimension keys by the specified partitions. </p>
        pub fn set_partition_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ResponsePartitionKey>>,
        ) -> Self {
            self.partition_keys = input;
            self
        }
        /// Appends an item to `keys`.
        ///
        /// To override the contents of this collection use [`set_keys`](Self::set_keys).
        ///
        /// <p>The dimension keys that were requested.</p>
        pub fn keys(mut self, input: crate::model::DimensionKeyDescription) -> Self {
            let mut v = self.keys.unwrap_or_default();
            v.push(input);
            self.keys = Some(v);
            self
        }
        /// <p>The dimension keys that were requested.</p>
        pub fn set_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::DimensionKeyDescription>>,
        ) -> Self {
            self.keys = input;
            self
        }
        /// <p>A pagination token that indicates the response didn’t return all available records because <code>MaxRecords</code> was specified in the previous request. To get the remaining records, specify <code>NextToken</code> in a separate request with this value. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A pagination token that indicates the response didn’t return all available records because <code>MaxRecords</code> was specified in the previous request. To get the remaining records, specify <code>NextToken</code> in a separate request with this value. </p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeDimensionKeysOutput`](crate::output::DescribeDimensionKeysOutput)
        pub fn build(self) -> crate::output::DescribeDimensionKeysOutput {
            crate::output::DescribeDimensionKeysOutput {
                aligned_start_time: self.aligned_start_time,
                aligned_end_time: self.aligned_end_time,
                partition_keys: self.partition_keys,
                keys: self.keys,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeDimensionKeysOutput {
    /// Creates a new builder-style object to manufacture [`DescribeDimensionKeysOutput`](crate::output::DescribeDimensionKeysOutput)
    pub fn builder() -> crate::output::describe_dimension_keys_output::Builder {
        crate::output::describe_dimension_keys_output::Builder::default()
    }
}
