// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateDeviceStateOutput {}
impl std::fmt::Debug for UpdateDeviceStateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateDeviceStateOutput");
        formatter.finish()
    }
}
/// See [`UpdateDeviceStateOutput`](crate::output::UpdateDeviceStateOutput)
pub mod update_device_state_output {

    /// A builder for [`UpdateDeviceStateOutput`](crate::output::UpdateDeviceStateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateDeviceStateOutput`](crate::output::UpdateDeviceStateOutput)
        pub fn build(self) -> crate::output::UpdateDeviceStateOutput {
            crate::output::UpdateDeviceStateOutput {}
        }
    }
}
impl UpdateDeviceStateOutput {
    /// Creates a new builder-style object to manufacture [`UpdateDeviceStateOutput`](crate::output::UpdateDeviceStateOutput)
    pub fn builder() -> crate::output::update_device_state_output::Builder {
        crate::output::update_device_state_output::Builder::default()
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
pub struct UnclaimDeviceOutput {
    /// <p>The device's final claim state.</p>
    pub state: std::option::Option<std::string::String>,
}
impl UnclaimDeviceOutput {
    /// <p>The device's final claim state.</p>
    pub fn state(&self) -> std::option::Option<&str> {
        self.state.as_deref()
    }
}
impl std::fmt::Debug for UnclaimDeviceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UnclaimDeviceOutput");
        formatter.field("state", &self.state);
        formatter.finish()
    }
}
/// See [`UnclaimDeviceOutput`](crate::output::UnclaimDeviceOutput)
pub mod unclaim_device_output {

    /// A builder for [`UnclaimDeviceOutput`](crate::output::UnclaimDeviceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) state: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The device's final claim state.</p>
        pub fn state(mut self, input: impl Into<std::string::String>) -> Self {
            self.state = Some(input.into());
            self
        }
        /// <p>The device's final claim state.</p>
        pub fn set_state(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.state = input;
            self
        }
        /// Consumes the builder and constructs a [`UnclaimDeviceOutput`](crate::output::UnclaimDeviceOutput)
        pub fn build(self) -> crate::output::UnclaimDeviceOutput {
            crate::output::UnclaimDeviceOutput { state: self.state }
        }
    }
}
impl UnclaimDeviceOutput {
    /// Creates a new builder-style object to manufacture [`UnclaimDeviceOutput`](crate::output::UnclaimDeviceOutput)
    pub fn builder() -> crate::output::unclaim_device_output::Builder {
        crate::output::unclaim_device_output::Builder::default()
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
pub struct ListTagsForResourceOutput {
    /// <p>A collection of key/value pairs defining the resource tags. For example, { "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p>
    /// <p> </p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ListTagsForResourceOutput {
    /// <p>A collection of key/value pairs defining the resource tags. For example, { "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p>
    /// <p> </p>
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
        /// <p>A collection of key/value pairs defining the resource tags. For example, { "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p>
        /// <p> </p>
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
        /// <p>A collection of key/value pairs defining the resource tags. For example, { "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p>
        /// <p> </p>
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
pub struct ListDevicesOutput {
    /// <p>A list of devices.</p>
    pub devices: std::option::Option<std::vec::Vec<crate::model::DeviceDescription>>,
    /// <p>The token to retrieve the next set of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListDevicesOutput {
    /// <p>A list of devices.</p>
    pub fn devices(&self) -> std::option::Option<&[crate::model::DeviceDescription]> {
        self.devices.as_deref()
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListDevicesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListDevicesOutput");
        formatter.field("devices", &self.devices);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListDevicesOutput`](crate::output::ListDevicesOutput)
pub mod list_devices_output {

    /// A builder for [`ListDevicesOutput`](crate::output::ListDevicesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) devices: std::option::Option<std::vec::Vec<crate::model::DeviceDescription>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `devices`.
        ///
        /// To override the contents of this collection use [`set_devices`](Self::set_devices).
        ///
        /// <p>A list of devices.</p>
        pub fn devices(mut self, input: crate::model::DeviceDescription) -> Self {
            let mut v = self.devices.unwrap_or_default();
            v.push(input);
            self.devices = Some(v);
            self
        }
        /// <p>A list of devices.</p>
        pub fn set_devices(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::DeviceDescription>>,
        ) -> Self {
            self.devices = input;
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListDevicesOutput`](crate::output::ListDevicesOutput)
        pub fn build(self) -> crate::output::ListDevicesOutput {
            crate::output::ListDevicesOutput {
                devices: self.devices,
                next_token: self.next_token,
            }
        }
    }
}
impl ListDevicesOutput {
    /// Creates a new builder-style object to manufacture [`ListDevicesOutput`](crate::output::ListDevicesOutput)
    pub fn builder() -> crate::output::list_devices_output::Builder {
        crate::output::list_devices_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListDeviceEventsOutput {
    /// <p>An array of zero or more elements describing the event(s) associated with the device.</p>
    pub events: std::option::Option<std::vec::Vec<crate::model::DeviceEvent>>,
    /// <p>The token to retrieve the next set of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListDeviceEventsOutput {
    /// <p>An array of zero or more elements describing the event(s) associated with the device.</p>
    pub fn events(&self) -> std::option::Option<&[crate::model::DeviceEvent]> {
        self.events.as_deref()
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListDeviceEventsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListDeviceEventsOutput");
        formatter.field("events", &self.events);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListDeviceEventsOutput`](crate::output::ListDeviceEventsOutput)
pub mod list_device_events_output {

    /// A builder for [`ListDeviceEventsOutput`](crate::output::ListDeviceEventsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) events: std::option::Option<std::vec::Vec<crate::model::DeviceEvent>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `events`.
        ///
        /// To override the contents of this collection use [`set_events`](Self::set_events).
        ///
        /// <p>An array of zero or more elements describing the event(s) associated with the device.</p>
        pub fn events(mut self, input: crate::model::DeviceEvent) -> Self {
            let mut v = self.events.unwrap_or_default();
            v.push(input);
            self.events = Some(v);
            self
        }
        /// <p>An array of zero or more elements describing the event(s) associated with the device.</p>
        pub fn set_events(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::DeviceEvent>>,
        ) -> Self {
            self.events = input;
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListDeviceEventsOutput`](crate::output::ListDeviceEventsOutput)
        pub fn build(self) -> crate::output::ListDeviceEventsOutput {
            crate::output::ListDeviceEventsOutput {
                events: self.events,
                next_token: self.next_token,
            }
        }
    }
}
impl ListDeviceEventsOutput {
    /// Creates a new builder-style object to manufacture [`ListDeviceEventsOutput`](crate::output::ListDeviceEventsOutput)
    pub fn builder() -> crate::output::list_device_events_output::Builder {
        crate::output::list_device_events_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvokeDeviceMethodOutput {
    /// <p>A JSON encoded string containing the device method response.</p>
    pub device_method_response: std::option::Option<std::string::String>,
}
impl InvokeDeviceMethodOutput {
    /// <p>A JSON encoded string containing the device method response.</p>
    pub fn device_method_response(&self) -> std::option::Option<&str> {
        self.device_method_response.as_deref()
    }
}
impl std::fmt::Debug for InvokeDeviceMethodOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvokeDeviceMethodOutput");
        formatter.field("device_method_response", &self.device_method_response);
        formatter.finish()
    }
}
/// See [`InvokeDeviceMethodOutput`](crate::output::InvokeDeviceMethodOutput)
pub mod invoke_device_method_output {

    /// A builder for [`InvokeDeviceMethodOutput`](crate::output::InvokeDeviceMethodOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) device_method_response: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>A JSON encoded string containing the device method response.</p>
        pub fn device_method_response(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_method_response = Some(input.into());
            self
        }
        /// <p>A JSON encoded string containing the device method response.</p>
        pub fn set_device_method_response(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.device_method_response = input;
            self
        }
        /// Consumes the builder and constructs a [`InvokeDeviceMethodOutput`](crate::output::InvokeDeviceMethodOutput)
        pub fn build(self) -> crate::output::InvokeDeviceMethodOutput {
            crate::output::InvokeDeviceMethodOutput {
                device_method_response: self.device_method_response,
            }
        }
    }
}
impl InvokeDeviceMethodOutput {
    /// Creates a new builder-style object to manufacture [`InvokeDeviceMethodOutput`](crate::output::InvokeDeviceMethodOutput)
    pub fn builder() -> crate::output::invoke_device_method_output::Builder {
        crate::output::invoke_device_method_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InitiateDeviceClaimOutput {
    /// <p>The device's final claim state.</p>
    pub state: std::option::Option<std::string::String>,
}
impl InitiateDeviceClaimOutput {
    /// <p>The device's final claim state.</p>
    pub fn state(&self) -> std::option::Option<&str> {
        self.state.as_deref()
    }
}
impl std::fmt::Debug for InitiateDeviceClaimOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InitiateDeviceClaimOutput");
        formatter.field("state", &self.state);
        formatter.finish()
    }
}
/// See [`InitiateDeviceClaimOutput`](crate::output::InitiateDeviceClaimOutput)
pub mod initiate_device_claim_output {

    /// A builder for [`InitiateDeviceClaimOutput`](crate::output::InitiateDeviceClaimOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) state: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The device's final claim state.</p>
        pub fn state(mut self, input: impl Into<std::string::String>) -> Self {
            self.state = Some(input.into());
            self
        }
        /// <p>The device's final claim state.</p>
        pub fn set_state(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.state = input;
            self
        }
        /// Consumes the builder and constructs a [`InitiateDeviceClaimOutput`](crate::output::InitiateDeviceClaimOutput)
        pub fn build(self) -> crate::output::InitiateDeviceClaimOutput {
            crate::output::InitiateDeviceClaimOutput { state: self.state }
        }
    }
}
impl InitiateDeviceClaimOutput {
    /// Creates a new builder-style object to manufacture [`InitiateDeviceClaimOutput`](crate::output::InitiateDeviceClaimOutput)
    pub fn builder() -> crate::output::initiate_device_claim_output::Builder {
        crate::output::initiate_device_claim_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetDeviceMethodsOutput {
    /// <p>List of available device APIs.</p>
    pub device_methods: std::option::Option<std::vec::Vec<crate::model::DeviceMethod>>,
}
impl GetDeviceMethodsOutput {
    /// <p>List of available device APIs.</p>
    pub fn device_methods(&self) -> std::option::Option<&[crate::model::DeviceMethod]> {
        self.device_methods.as_deref()
    }
}
impl std::fmt::Debug for GetDeviceMethodsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetDeviceMethodsOutput");
        formatter.field("device_methods", &self.device_methods);
        formatter.finish()
    }
}
/// See [`GetDeviceMethodsOutput`](crate::output::GetDeviceMethodsOutput)
pub mod get_device_methods_output {

    /// A builder for [`GetDeviceMethodsOutput`](crate::output::GetDeviceMethodsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) device_methods: std::option::Option<std::vec::Vec<crate::model::DeviceMethod>>,
    }
    impl Builder {
        /// Appends an item to `device_methods`.
        ///
        /// To override the contents of this collection use [`set_device_methods`](Self::set_device_methods).
        ///
        /// <p>List of available device APIs.</p>
        pub fn device_methods(mut self, input: crate::model::DeviceMethod) -> Self {
            let mut v = self.device_methods.unwrap_or_default();
            v.push(input);
            self.device_methods = Some(v);
            self
        }
        /// <p>List of available device APIs.</p>
        pub fn set_device_methods(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::DeviceMethod>>,
        ) -> Self {
            self.device_methods = input;
            self
        }
        /// Consumes the builder and constructs a [`GetDeviceMethodsOutput`](crate::output::GetDeviceMethodsOutput)
        pub fn build(self) -> crate::output::GetDeviceMethodsOutput {
            crate::output::GetDeviceMethodsOutput {
                device_methods: self.device_methods,
            }
        }
    }
}
impl GetDeviceMethodsOutput {
    /// Creates a new builder-style object to manufacture [`GetDeviceMethodsOutput`](crate::output::GetDeviceMethodsOutput)
    pub fn builder() -> crate::output::get_device_methods_output::Builder {
        crate::output::get_device_methods_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct FinalizeDeviceClaimOutput {
    /// <p>The device's final claim state.</p>
    pub state: std::option::Option<std::string::String>,
}
impl FinalizeDeviceClaimOutput {
    /// <p>The device's final claim state.</p>
    pub fn state(&self) -> std::option::Option<&str> {
        self.state.as_deref()
    }
}
impl std::fmt::Debug for FinalizeDeviceClaimOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("FinalizeDeviceClaimOutput");
        formatter.field("state", &self.state);
        formatter.finish()
    }
}
/// See [`FinalizeDeviceClaimOutput`](crate::output::FinalizeDeviceClaimOutput)
pub mod finalize_device_claim_output {

    /// A builder for [`FinalizeDeviceClaimOutput`](crate::output::FinalizeDeviceClaimOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) state: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The device's final claim state.</p>
        pub fn state(mut self, input: impl Into<std::string::String>) -> Self {
            self.state = Some(input.into());
            self
        }
        /// <p>The device's final claim state.</p>
        pub fn set_state(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.state = input;
            self
        }
        /// Consumes the builder and constructs a [`FinalizeDeviceClaimOutput`](crate::output::FinalizeDeviceClaimOutput)
        pub fn build(self) -> crate::output::FinalizeDeviceClaimOutput {
            crate::output::FinalizeDeviceClaimOutput { state: self.state }
        }
    }
}
impl FinalizeDeviceClaimOutput {
    /// Creates a new builder-style object to manufacture [`FinalizeDeviceClaimOutput`](crate::output::FinalizeDeviceClaimOutput)
    pub fn builder() -> crate::output::finalize_device_claim_output::Builder {
        crate::output::finalize_device_claim_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeDeviceOutput {
    /// <p>Device details.</p>
    pub device_description: std::option::Option<crate::model::DeviceDescription>,
}
impl DescribeDeviceOutput {
    /// <p>Device details.</p>
    pub fn device_description(&self) -> std::option::Option<&crate::model::DeviceDescription> {
        self.device_description.as_ref()
    }
}
impl std::fmt::Debug for DescribeDeviceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeDeviceOutput");
        formatter.field("device_description", &self.device_description);
        formatter.finish()
    }
}
/// See [`DescribeDeviceOutput`](crate::output::DescribeDeviceOutput)
pub mod describe_device_output {

    /// A builder for [`DescribeDeviceOutput`](crate::output::DescribeDeviceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) device_description: std::option::Option<crate::model::DeviceDescription>,
    }
    impl Builder {
        /// <p>Device details.</p>
        pub fn device_description(mut self, input: crate::model::DeviceDescription) -> Self {
            self.device_description = Some(input);
            self
        }
        /// <p>Device details.</p>
        pub fn set_device_description(
            mut self,
            input: std::option::Option<crate::model::DeviceDescription>,
        ) -> Self {
            self.device_description = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeDeviceOutput`](crate::output::DescribeDeviceOutput)
        pub fn build(self) -> crate::output::DescribeDeviceOutput {
            crate::output::DescribeDeviceOutput {
                device_description: self.device_description,
            }
        }
    }
}
impl DescribeDeviceOutput {
    /// Creates a new builder-style object to manufacture [`DescribeDeviceOutput`](crate::output::DescribeDeviceOutput)
    pub fn builder() -> crate::output::describe_device_output::Builder {
        crate::output::describe_device_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ClaimDevicesByClaimCodeOutput {
    /// <p>The claim code provided by the device manufacturer.</p>
    pub claim_code: std::option::Option<std::string::String>,
    /// <p>The total number of devices associated with the claim code that has been processed in the claim request.</p>
    pub total: i32,
}
impl ClaimDevicesByClaimCodeOutput {
    /// <p>The claim code provided by the device manufacturer.</p>
    pub fn claim_code(&self) -> std::option::Option<&str> {
        self.claim_code.as_deref()
    }
    /// <p>The total number of devices associated with the claim code that has been processed in the claim request.</p>
    pub fn total(&self) -> i32 {
        self.total
    }
}
impl std::fmt::Debug for ClaimDevicesByClaimCodeOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ClaimDevicesByClaimCodeOutput");
        formatter.field("claim_code", &self.claim_code);
        formatter.field("total", &self.total);
        formatter.finish()
    }
}
/// See [`ClaimDevicesByClaimCodeOutput`](crate::output::ClaimDevicesByClaimCodeOutput)
pub mod claim_devices_by_claim_code_output {

    /// A builder for [`ClaimDevicesByClaimCodeOutput`](crate::output::ClaimDevicesByClaimCodeOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) claim_code: std::option::Option<std::string::String>,
        pub(crate) total: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The claim code provided by the device manufacturer.</p>
        pub fn claim_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.claim_code = Some(input.into());
            self
        }
        /// <p>The claim code provided by the device manufacturer.</p>
        pub fn set_claim_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.claim_code = input;
            self
        }
        /// <p>The total number of devices associated with the claim code that has been processed in the claim request.</p>
        pub fn total(mut self, input: i32) -> Self {
            self.total = Some(input);
            self
        }
        /// <p>The total number of devices associated with the claim code that has been processed in the claim request.</p>
        pub fn set_total(mut self, input: std::option::Option<i32>) -> Self {
            self.total = input;
            self
        }
        /// Consumes the builder and constructs a [`ClaimDevicesByClaimCodeOutput`](crate::output::ClaimDevicesByClaimCodeOutput)
        pub fn build(self) -> crate::output::ClaimDevicesByClaimCodeOutput {
            crate::output::ClaimDevicesByClaimCodeOutput {
                claim_code: self.claim_code,
                total: self.total.unwrap_or_default(),
            }
        }
    }
}
impl ClaimDevicesByClaimCodeOutput {
    /// Creates a new builder-style object to manufacture [`ClaimDevicesByClaimCodeOutput`](crate::output::ClaimDevicesByClaimCodeOutput)
    pub fn builder() -> crate::output::claim_devices_by_claim_code_output::Builder {
        crate::output::claim_devices_by_claim_code_output::Builder::default()
    }
}
