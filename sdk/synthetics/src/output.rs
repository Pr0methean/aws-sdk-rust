// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateCanaryOutput {}
impl std::fmt::Debug for UpdateCanaryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateCanaryOutput");
        formatter.finish()
    }
}
/// See [`UpdateCanaryOutput`](crate::output::UpdateCanaryOutput)
pub mod update_canary_output {

    /// A builder for [`UpdateCanaryOutput`](crate::output::UpdateCanaryOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateCanaryOutput`](crate::output::UpdateCanaryOutput)
        pub fn build(self) -> crate::output::UpdateCanaryOutput {
            crate::output::UpdateCanaryOutput {}
        }
    }
}
impl UpdateCanaryOutput {
    /// Creates a new builder-style object to manufacture [`UpdateCanaryOutput`](crate::output::UpdateCanaryOutput)
    pub fn builder() -> crate::output::update_canary_output::Builder {
        crate::output::update_canary_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UntagResourceOutput {}
impl std::fmt::Debug for UntagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UntagResourceOutput");
        formatter.finish()
    }
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput)
pub mod untag_resource_output {

    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput)
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TagResourceOutput {}
impl std::fmt::Debug for TagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TagResourceOutput");
        formatter.finish()
    }
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput)
pub mod tag_resource_output {

    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput)
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput)
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StopCanaryOutput {}
impl std::fmt::Debug for StopCanaryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StopCanaryOutput");
        formatter.finish()
    }
}
/// See [`StopCanaryOutput`](crate::output::StopCanaryOutput)
pub mod stop_canary_output {

    /// A builder for [`StopCanaryOutput`](crate::output::StopCanaryOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`StopCanaryOutput`](crate::output::StopCanaryOutput)
        pub fn build(self) -> crate::output::StopCanaryOutput {
            crate::output::StopCanaryOutput {}
        }
    }
}
impl StopCanaryOutput {
    /// Creates a new builder-style object to manufacture [`StopCanaryOutput`](crate::output::StopCanaryOutput)
    pub fn builder() -> crate::output::stop_canary_output::Builder {
        crate::output::stop_canary_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartCanaryOutput {}
impl std::fmt::Debug for StartCanaryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartCanaryOutput");
        formatter.finish()
    }
}
/// See [`StartCanaryOutput`](crate::output::StartCanaryOutput)
pub mod start_canary_output {

    /// A builder for [`StartCanaryOutput`](crate::output::StartCanaryOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`StartCanaryOutput`](crate::output::StartCanaryOutput)
        pub fn build(self) -> crate::output::StartCanaryOutput {
            crate::output::StartCanaryOutput {}
        }
    }
}
impl StartCanaryOutput {
    /// Creates a new builder-style object to manufacture [`StartCanaryOutput`](crate::output::StartCanaryOutput)
    pub fn builder() -> crate::output::start_canary_output::Builder {
        crate::output::start_canary_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTagsForResourceOutput {
    /// <p>The list of tag keys and values associated with the canary that you specified.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ListTagsForResourceOutput {
    /// <p>The list of tag keys and values associated with the canary that you specified.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl std::fmt::Debug for ListTagsForResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTagsForResourceOutput");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
pub mod list_tags_for_resource_output {

    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The list of tag keys and values associated with the canary that you specified.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>The list of tag keys and values associated with the canary that you specified.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput { tags: self.tags }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetCanaryRunsOutput {
    /// <p>An array of structures. Each structure contains the details of one of the retrieved canary runs.</p>
    pub canary_runs: std::option::Option<std::vec::Vec<crate::model::CanaryRun>>,
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>GetCanaryRuns</code> operation to retrieve the next set of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl GetCanaryRunsOutput {
    /// <p>An array of structures. Each structure contains the details of one of the retrieved canary runs.</p>
    pub fn canary_runs(&self) -> std::option::Option<&[crate::model::CanaryRun]> {
        self.canary_runs.as_deref()
    }
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>GetCanaryRuns</code> operation to retrieve the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for GetCanaryRunsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetCanaryRunsOutput");
        formatter.field("canary_runs", &self.canary_runs);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`GetCanaryRunsOutput`](crate::output::GetCanaryRunsOutput)
pub mod get_canary_runs_output {

    /// A builder for [`GetCanaryRunsOutput`](crate::output::GetCanaryRunsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) canary_runs: std::option::Option<std::vec::Vec<crate::model::CanaryRun>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `canary_runs`.
        ///
        /// To override the contents of this collection use [`set_canary_runs`](Self::set_canary_runs).
        ///
        /// <p>An array of structures. Each structure contains the details of one of the retrieved canary runs.</p>
        pub fn canary_runs(mut self, input: crate::model::CanaryRun) -> Self {
            let mut v = self.canary_runs.unwrap_or_default();
            v.push(input);
            self.canary_runs = Some(v);
            self
        }
        /// <p>An array of structures. Each structure contains the details of one of the retrieved canary runs.</p>
        pub fn set_canary_runs(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::CanaryRun>>,
        ) -> Self {
            self.canary_runs = input;
            self
        }
        /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>GetCanaryRuns</code> operation to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>GetCanaryRuns</code> operation to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`GetCanaryRunsOutput`](crate::output::GetCanaryRunsOutput)
        pub fn build(self) -> crate::output::GetCanaryRunsOutput {
            crate::output::GetCanaryRunsOutput {
                canary_runs: self.canary_runs,
                next_token: self.next_token,
            }
        }
    }
}
impl GetCanaryRunsOutput {
    /// Creates a new builder-style object to manufacture [`GetCanaryRunsOutput`](crate::output::GetCanaryRunsOutput)
    pub fn builder() -> crate::output::get_canary_runs_output::Builder {
        crate::output::get_canary_runs_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetCanaryOutput {
    /// <p>A strucure that contains the full information about the canary.</p>
    pub canary: std::option::Option<crate::model::Canary>,
}
impl GetCanaryOutput {
    /// <p>A strucure that contains the full information about the canary.</p>
    pub fn canary(&self) -> std::option::Option<&crate::model::Canary> {
        self.canary.as_ref()
    }
}
impl std::fmt::Debug for GetCanaryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetCanaryOutput");
        formatter.field("canary", &self.canary);
        formatter.finish()
    }
}
/// See [`GetCanaryOutput`](crate::output::GetCanaryOutput)
pub mod get_canary_output {

    /// A builder for [`GetCanaryOutput`](crate::output::GetCanaryOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) canary: std::option::Option<crate::model::Canary>,
    }
    impl Builder {
        /// <p>A strucure that contains the full information about the canary.</p>
        pub fn canary(mut self, input: crate::model::Canary) -> Self {
            self.canary = Some(input);
            self
        }
        /// <p>A strucure that contains the full information about the canary.</p>
        pub fn set_canary(mut self, input: std::option::Option<crate::model::Canary>) -> Self {
            self.canary = input;
            self
        }
        /// Consumes the builder and constructs a [`GetCanaryOutput`](crate::output::GetCanaryOutput)
        pub fn build(self) -> crate::output::GetCanaryOutput {
            crate::output::GetCanaryOutput {
                canary: self.canary,
            }
        }
    }
}
impl GetCanaryOutput {
    /// Creates a new builder-style object to manufacture [`GetCanaryOutput`](crate::output::GetCanaryOutput)
    pub fn builder() -> crate::output::get_canary_output::Builder {
        crate::output::get_canary_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeRuntimeVersionsOutput {
    /// <p>An array of objects that display the details about each Synthetics canary runtime version.</p>
    pub runtime_versions: std::option::Option<std::vec::Vec<crate::model::RuntimeVersion>>,
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeRuntimeVersions</code> operation to retrieve the next set of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeRuntimeVersionsOutput {
    /// <p>An array of objects that display the details about each Synthetics canary runtime version.</p>
    pub fn runtime_versions(&self) -> std::option::Option<&[crate::model::RuntimeVersion]> {
        self.runtime_versions.as_deref()
    }
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeRuntimeVersions</code> operation to retrieve the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for DescribeRuntimeVersionsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeRuntimeVersionsOutput");
        formatter.field("runtime_versions", &self.runtime_versions);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeRuntimeVersionsOutput`](crate::output::DescribeRuntimeVersionsOutput)
pub mod describe_runtime_versions_output {

    /// A builder for [`DescribeRuntimeVersionsOutput`](crate::output::DescribeRuntimeVersionsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) runtime_versions:
            std::option::Option<std::vec::Vec<crate::model::RuntimeVersion>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `runtime_versions`.
        ///
        /// To override the contents of this collection use [`set_runtime_versions`](Self::set_runtime_versions).
        ///
        /// <p>An array of objects that display the details about each Synthetics canary runtime version.</p>
        pub fn runtime_versions(mut self, input: crate::model::RuntimeVersion) -> Self {
            let mut v = self.runtime_versions.unwrap_or_default();
            v.push(input);
            self.runtime_versions = Some(v);
            self
        }
        /// <p>An array of objects that display the details about each Synthetics canary runtime version.</p>
        pub fn set_runtime_versions(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::RuntimeVersion>>,
        ) -> Self {
            self.runtime_versions = input;
            self
        }
        /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeRuntimeVersions</code> operation to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeRuntimeVersions</code> operation to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeRuntimeVersionsOutput`](crate::output::DescribeRuntimeVersionsOutput)
        pub fn build(self) -> crate::output::DescribeRuntimeVersionsOutput {
            crate::output::DescribeRuntimeVersionsOutput {
                runtime_versions: self.runtime_versions,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeRuntimeVersionsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeRuntimeVersionsOutput`](crate::output::DescribeRuntimeVersionsOutput)
    pub fn builder() -> crate::output::describe_runtime_versions_output::Builder {
        crate::output::describe_runtime_versions_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeCanariesLastRunOutput {
    /// <p>An array that contains the information from the most recent run of each canary.</p>
    pub canaries_last_run: std::option::Option<std::vec::Vec<crate::model::CanaryLastRun>>,
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeCanariesLastRun</code> operation to retrieve the next set of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeCanariesLastRunOutput {
    /// <p>An array that contains the information from the most recent run of each canary.</p>
    pub fn canaries_last_run(&self) -> std::option::Option<&[crate::model::CanaryLastRun]> {
        self.canaries_last_run.as_deref()
    }
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeCanariesLastRun</code> operation to retrieve the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for DescribeCanariesLastRunOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeCanariesLastRunOutput");
        formatter.field("canaries_last_run", &self.canaries_last_run);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeCanariesLastRunOutput`](crate::output::DescribeCanariesLastRunOutput)
pub mod describe_canaries_last_run_output {

    /// A builder for [`DescribeCanariesLastRunOutput`](crate::output::DescribeCanariesLastRunOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) canaries_last_run:
            std::option::Option<std::vec::Vec<crate::model::CanaryLastRun>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `canaries_last_run`.
        ///
        /// To override the contents of this collection use [`set_canaries_last_run`](Self::set_canaries_last_run).
        ///
        /// <p>An array that contains the information from the most recent run of each canary.</p>
        pub fn canaries_last_run(mut self, input: crate::model::CanaryLastRun) -> Self {
            let mut v = self.canaries_last_run.unwrap_or_default();
            v.push(input);
            self.canaries_last_run = Some(v);
            self
        }
        /// <p>An array that contains the information from the most recent run of each canary.</p>
        pub fn set_canaries_last_run(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::CanaryLastRun>>,
        ) -> Self {
            self.canaries_last_run = input;
            self
        }
        /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeCanariesLastRun</code> operation to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeCanariesLastRun</code> operation to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeCanariesLastRunOutput`](crate::output::DescribeCanariesLastRunOutput)
        pub fn build(self) -> crate::output::DescribeCanariesLastRunOutput {
            crate::output::DescribeCanariesLastRunOutput {
                canaries_last_run: self.canaries_last_run,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeCanariesLastRunOutput {
    /// Creates a new builder-style object to manufacture [`DescribeCanariesLastRunOutput`](crate::output::DescribeCanariesLastRunOutput)
    pub fn builder() -> crate::output::describe_canaries_last_run_output::Builder {
        crate::output::describe_canaries_last_run_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeCanariesOutput {
    /// <p>Returns an array. Each item in the array contains the full information about one canary.</p>
    pub canaries: std::option::Option<std::vec::Vec<crate::model::Canary>>,
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeCanaries</code> operation to retrieve the next set of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeCanariesOutput {
    /// <p>Returns an array. Each item in the array contains the full information about one canary.</p>
    pub fn canaries(&self) -> std::option::Option<&[crate::model::Canary]> {
        self.canaries.as_deref()
    }
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeCanaries</code> operation to retrieve the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for DescribeCanariesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeCanariesOutput");
        formatter.field("canaries", &self.canaries);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeCanariesOutput`](crate::output::DescribeCanariesOutput)
pub mod describe_canaries_output {

    /// A builder for [`DescribeCanariesOutput`](crate::output::DescribeCanariesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) canaries: std::option::Option<std::vec::Vec<crate::model::Canary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `canaries`.
        ///
        /// To override the contents of this collection use [`set_canaries`](Self::set_canaries).
        ///
        /// <p>Returns an array. Each item in the array contains the full information about one canary.</p>
        pub fn canaries(mut self, input: crate::model::Canary) -> Self {
            let mut v = self.canaries.unwrap_or_default();
            v.push(input);
            self.canaries = Some(v);
            self
        }
        /// <p>Returns an array. Each item in the array contains the full information about one canary.</p>
        pub fn set_canaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Canary>>,
        ) -> Self {
            self.canaries = input;
            self
        }
        /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeCanaries</code> operation to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>DescribeCanaries</code> operation to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeCanariesOutput`](crate::output::DescribeCanariesOutput)
        pub fn build(self) -> crate::output::DescribeCanariesOutput {
            crate::output::DescribeCanariesOutput {
                canaries: self.canaries,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeCanariesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeCanariesOutput`](crate::output::DescribeCanariesOutput)
    pub fn builder() -> crate::output::describe_canaries_output::Builder {
        crate::output::describe_canaries_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteCanaryOutput {}
impl std::fmt::Debug for DeleteCanaryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteCanaryOutput");
        formatter.finish()
    }
}
/// See [`DeleteCanaryOutput`](crate::output::DeleteCanaryOutput)
pub mod delete_canary_output {

    /// A builder for [`DeleteCanaryOutput`](crate::output::DeleteCanaryOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteCanaryOutput`](crate::output::DeleteCanaryOutput)
        pub fn build(self) -> crate::output::DeleteCanaryOutput {
            crate::output::DeleteCanaryOutput {}
        }
    }
}
impl DeleteCanaryOutput {
    /// Creates a new builder-style object to manufacture [`DeleteCanaryOutput`](crate::output::DeleteCanaryOutput)
    pub fn builder() -> crate::output::delete_canary_output::Builder {
        crate::output::delete_canary_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateCanaryOutput {
    /// <p>The full details about the canary you have created.</p>
    pub canary: std::option::Option<crate::model::Canary>,
}
impl CreateCanaryOutput {
    /// <p>The full details about the canary you have created.</p>
    pub fn canary(&self) -> std::option::Option<&crate::model::Canary> {
        self.canary.as_ref()
    }
}
impl std::fmt::Debug for CreateCanaryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateCanaryOutput");
        formatter.field("canary", &self.canary);
        formatter.finish()
    }
}
/// See [`CreateCanaryOutput`](crate::output::CreateCanaryOutput)
pub mod create_canary_output {

    /// A builder for [`CreateCanaryOutput`](crate::output::CreateCanaryOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) canary: std::option::Option<crate::model::Canary>,
    }
    impl Builder {
        /// <p>The full details about the canary you have created.</p>
        pub fn canary(mut self, input: crate::model::Canary) -> Self {
            self.canary = Some(input);
            self
        }
        /// <p>The full details about the canary you have created.</p>
        pub fn set_canary(mut self, input: std::option::Option<crate::model::Canary>) -> Self {
            self.canary = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateCanaryOutput`](crate::output::CreateCanaryOutput)
        pub fn build(self) -> crate::output::CreateCanaryOutput {
            crate::output::CreateCanaryOutput {
                canary: self.canary,
            }
        }
    }
}
impl CreateCanaryOutput {
    /// Creates a new builder-style object to manufacture [`CreateCanaryOutput`](crate::output::CreateCanaryOutput)
    pub fn builder() -> crate::output::create_canary_output::Builder {
        crate::output::create_canary_output::Builder::default()
    }
}
