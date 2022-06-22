// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The details of an Elastic Inference Accelerator type. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AcceleratorType {
    /// <p> The name of the Elastic Inference Accelerator type. </p>
    pub accelerator_type_name: std::option::Option<std::string::String>,
    /// <p> The memory information of the Elastic Inference Accelerator type. </p>
    pub memory_info: std::option::Option<crate::model::MemoryInfo>,
    /// <p> The throughput information of the Elastic Inference Accelerator type. </p>
    pub throughput_info: std::option::Option<std::vec::Vec<crate::model::KeyValuePair>>,
}
impl AcceleratorType {
    /// <p> The name of the Elastic Inference Accelerator type. </p>
    pub fn accelerator_type_name(&self) -> std::option::Option<&str> {
        self.accelerator_type_name.as_deref()
    }
    /// <p> The memory information of the Elastic Inference Accelerator type. </p>
    pub fn memory_info(&self) -> std::option::Option<&crate::model::MemoryInfo> {
        self.memory_info.as_ref()
    }
    /// <p> The throughput information of the Elastic Inference Accelerator type. </p>
    pub fn throughput_info(&self) -> std::option::Option<&[crate::model::KeyValuePair]> {
        self.throughput_info.as_deref()
    }
}
impl std::fmt::Debug for AcceleratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AcceleratorType");
        formatter.field("accelerator_type_name", &self.accelerator_type_name);
        formatter.field("memory_info", &self.memory_info);
        formatter.field("throughput_info", &self.throughput_info);
        formatter.finish()
    }
}
/// See [`AcceleratorType`](crate::model::AcceleratorType)
pub mod accelerator_type {

    /// A builder for [`AcceleratorType`](crate::model::AcceleratorType)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) accelerator_type_name: std::option::Option<std::string::String>,
        pub(crate) memory_info: std::option::Option<crate::model::MemoryInfo>,
        pub(crate) throughput_info: std::option::Option<std::vec::Vec<crate::model::KeyValuePair>>,
    }
    impl Builder {
        /// <p> The name of the Elastic Inference Accelerator type. </p>
        pub fn accelerator_type_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.accelerator_type_name = Some(input.into());
            self
        }
        /// <p> The name of the Elastic Inference Accelerator type. </p>
        pub fn set_accelerator_type_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.accelerator_type_name = input;
            self
        }
        /// <p> The memory information of the Elastic Inference Accelerator type. </p>
        pub fn memory_info(mut self, input: crate::model::MemoryInfo) -> Self {
            self.memory_info = Some(input);
            self
        }
        /// <p> The memory information of the Elastic Inference Accelerator type. </p>
        pub fn set_memory_info(
            mut self,
            input: std::option::Option<crate::model::MemoryInfo>,
        ) -> Self {
            self.memory_info = input;
            self
        }
        /// Appends an item to `throughput_info`.
        ///
        /// To override the contents of this collection use [`set_throughput_info`](Self::set_throughput_info).
        ///
        /// <p> The throughput information of the Elastic Inference Accelerator type. </p>
        pub fn throughput_info(mut self, input: crate::model::KeyValuePair) -> Self {
            let mut v = self.throughput_info.unwrap_or_default();
            v.push(input);
            self.throughput_info = Some(v);
            self
        }
        /// <p> The throughput information of the Elastic Inference Accelerator type. </p>
        pub fn set_throughput_info(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::KeyValuePair>>,
        ) -> Self {
            self.throughput_info = input;
            self
        }
        /// Consumes the builder and constructs a [`AcceleratorType`](crate::model::AcceleratorType)
        pub fn build(self) -> crate::model::AcceleratorType {
            crate::model::AcceleratorType {
                accelerator_type_name: self.accelerator_type_name,
                memory_info: self.memory_info,
                throughput_info: self.throughput_info,
            }
        }
    }
}
impl AcceleratorType {
    /// Creates a new builder-style object to manufacture [`AcceleratorType`](crate::model::AcceleratorType)
    pub fn builder() -> crate::model::accelerator_type::Builder {
        crate::model::accelerator_type::Builder::default()
    }
}

/// <p> A throughput entry for an Elastic Inference Accelerator type. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct KeyValuePair {
    /// <p> The throughput value of the Elastic Inference Accelerator type. It can assume the following values: TFLOPS16bit: the throughput expressed in 16bit TeraFLOPS. TFLOPS32bit: the throughput expressed in 32bit TeraFLOPS. </p>
    pub key: std::option::Option<std::string::String>,
    /// <p> The throughput value of the Elastic Inference Accelerator type. </p>
    pub value: i32,
}
impl KeyValuePair {
    /// <p> The throughput value of the Elastic Inference Accelerator type. It can assume the following values: TFLOPS16bit: the throughput expressed in 16bit TeraFLOPS. TFLOPS32bit: the throughput expressed in 32bit TeraFLOPS. </p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p> The throughput value of the Elastic Inference Accelerator type. </p>
    pub fn value(&self) -> i32 {
        self.value
    }
}
impl std::fmt::Debug for KeyValuePair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("KeyValuePair");
        formatter.field("key", &self.key);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`KeyValuePair`](crate::model::KeyValuePair)
pub mod key_value_pair {

    /// A builder for [`KeyValuePair`](crate::model::KeyValuePair)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<i32>,
    }
    impl Builder {
        /// <p> The throughput value of the Elastic Inference Accelerator type. It can assume the following values: TFLOPS16bit: the throughput expressed in 16bit TeraFLOPS. TFLOPS32bit: the throughput expressed in 32bit TeraFLOPS. </p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        /// <p> The throughput value of the Elastic Inference Accelerator type. It can assume the following values: TFLOPS16bit: the throughput expressed in 16bit TeraFLOPS. TFLOPS32bit: the throughput expressed in 32bit TeraFLOPS. </p>
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p> The throughput value of the Elastic Inference Accelerator type. </p>
        pub fn value(mut self, input: i32) -> Self {
            self.value = Some(input);
            self
        }
        /// <p> The throughput value of the Elastic Inference Accelerator type. </p>
        pub fn set_value(mut self, input: std::option::Option<i32>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`KeyValuePair`](crate::model::KeyValuePair)
        pub fn build(self) -> crate::model::KeyValuePair {
            crate::model::KeyValuePair {
                key: self.key,
                value: self.value.unwrap_or_default(),
            }
        }
    }
}
impl KeyValuePair {
    /// Creates a new builder-style object to manufacture [`KeyValuePair`](crate::model::KeyValuePair)
    pub fn builder() -> crate::model::key_value_pair::Builder {
        crate::model::key_value_pair::Builder::default()
    }
}

/// <p> The memory information of an Elastic Inference Accelerator type. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct MemoryInfo {
    /// <p> The size in mebibytes of the Elastic Inference Accelerator type. </p>
    pub size_in_mi_b: i32,
}
impl MemoryInfo {
    /// <p> The size in mebibytes of the Elastic Inference Accelerator type. </p>
    pub fn size_in_mi_b(&self) -> i32 {
        self.size_in_mi_b
    }
}
impl std::fmt::Debug for MemoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("MemoryInfo");
        formatter.field("size_in_mi_b", &self.size_in_mi_b);
        formatter.finish()
    }
}
/// See [`MemoryInfo`](crate::model::MemoryInfo)
pub mod memory_info {

    /// A builder for [`MemoryInfo`](crate::model::MemoryInfo)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) size_in_mi_b: std::option::Option<i32>,
    }
    impl Builder {
        /// <p> The size in mebibytes of the Elastic Inference Accelerator type. </p>
        pub fn size_in_mi_b(mut self, input: i32) -> Self {
            self.size_in_mi_b = Some(input);
            self
        }
        /// <p> The size in mebibytes of the Elastic Inference Accelerator type. </p>
        pub fn set_size_in_mi_b(mut self, input: std::option::Option<i32>) -> Self {
            self.size_in_mi_b = input;
            self
        }
        /// Consumes the builder and constructs a [`MemoryInfo`](crate::model::MemoryInfo)
        pub fn build(self) -> crate::model::MemoryInfo {
            crate::model::MemoryInfo {
                size_in_mi_b: self.size_in_mi_b.unwrap_or_default(),
            }
        }
    }
}
impl MemoryInfo {
    /// Creates a new builder-style object to manufacture [`MemoryInfo`](crate::model::MemoryInfo)
    pub fn builder() -> crate::model::memory_info::Builder {
        crate::model::memory_info::Builder::default()
    }
}

/// <p> The details of an Elastic Inference Accelerator. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ElasticInferenceAccelerator {
    /// <p> The health of the Elastic Inference Accelerator. </p>
    pub accelerator_health: std::option::Option<crate::model::ElasticInferenceAcceleratorHealth>,
    /// <p> The type of the Elastic Inference Accelerator. </p>
    pub accelerator_type: std::option::Option<std::string::String>,
    /// <p> The ID of the Elastic Inference Accelerator. </p>
    pub accelerator_id: std::option::Option<std::string::String>,
    /// <p> The availability zone where the Elastic Inference Accelerator is present. </p>
    pub availability_zone: std::option::Option<std::string::String>,
    /// <p> The ARN of the resource that the Elastic Inference Accelerator is attached to. </p>
    pub attached_resource: std::option::Option<std::string::String>,
}
impl ElasticInferenceAccelerator {
    /// <p> The health of the Elastic Inference Accelerator. </p>
    pub fn accelerator_health(
        &self,
    ) -> std::option::Option<&crate::model::ElasticInferenceAcceleratorHealth> {
        self.accelerator_health.as_ref()
    }
    /// <p> The type of the Elastic Inference Accelerator. </p>
    pub fn accelerator_type(&self) -> std::option::Option<&str> {
        self.accelerator_type.as_deref()
    }
    /// <p> The ID of the Elastic Inference Accelerator. </p>
    pub fn accelerator_id(&self) -> std::option::Option<&str> {
        self.accelerator_id.as_deref()
    }
    /// <p> The availability zone where the Elastic Inference Accelerator is present. </p>
    pub fn availability_zone(&self) -> std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p> The ARN of the resource that the Elastic Inference Accelerator is attached to. </p>
    pub fn attached_resource(&self) -> std::option::Option<&str> {
        self.attached_resource.as_deref()
    }
}
impl std::fmt::Debug for ElasticInferenceAccelerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ElasticInferenceAccelerator");
        formatter.field("accelerator_health", &self.accelerator_health);
        formatter.field("accelerator_type", &self.accelerator_type);
        formatter.field("accelerator_id", &self.accelerator_id);
        formatter.field("availability_zone", &self.availability_zone);
        formatter.field("attached_resource", &self.attached_resource);
        formatter.finish()
    }
}
/// See [`ElasticInferenceAccelerator`](crate::model::ElasticInferenceAccelerator)
pub mod elastic_inference_accelerator {

    /// A builder for [`ElasticInferenceAccelerator`](crate::model::ElasticInferenceAccelerator)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) accelerator_health:
            std::option::Option<crate::model::ElasticInferenceAcceleratorHealth>,
        pub(crate) accelerator_type: std::option::Option<std::string::String>,
        pub(crate) accelerator_id: std::option::Option<std::string::String>,
        pub(crate) availability_zone: std::option::Option<std::string::String>,
        pub(crate) attached_resource: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p> The health of the Elastic Inference Accelerator. </p>
        pub fn accelerator_health(
            mut self,
            input: crate::model::ElasticInferenceAcceleratorHealth,
        ) -> Self {
            self.accelerator_health = Some(input);
            self
        }
        /// <p> The health of the Elastic Inference Accelerator. </p>
        pub fn set_accelerator_health(
            mut self,
            input: std::option::Option<crate::model::ElasticInferenceAcceleratorHealth>,
        ) -> Self {
            self.accelerator_health = input;
            self
        }
        /// <p> The type of the Elastic Inference Accelerator. </p>
        pub fn accelerator_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.accelerator_type = Some(input.into());
            self
        }
        /// <p> The type of the Elastic Inference Accelerator. </p>
        pub fn set_accelerator_type(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.accelerator_type = input;
            self
        }
        /// <p> The ID of the Elastic Inference Accelerator. </p>
        pub fn accelerator_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.accelerator_id = Some(input.into());
            self
        }
        /// <p> The ID of the Elastic Inference Accelerator. </p>
        pub fn set_accelerator_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.accelerator_id = input;
            self
        }
        /// <p> The availability zone where the Elastic Inference Accelerator is present. </p>
        pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
            self.availability_zone = Some(input.into());
            self
        }
        /// <p> The availability zone where the Elastic Inference Accelerator is present. </p>
        pub fn set_availability_zone(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.availability_zone = input;
            self
        }
        /// <p> The ARN of the resource that the Elastic Inference Accelerator is attached to. </p>
        pub fn attached_resource(mut self, input: impl Into<std::string::String>) -> Self {
            self.attached_resource = Some(input.into());
            self
        }
        /// <p> The ARN of the resource that the Elastic Inference Accelerator is attached to. </p>
        pub fn set_attached_resource(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.attached_resource = input;
            self
        }
        /// Consumes the builder and constructs a [`ElasticInferenceAccelerator`](crate::model::ElasticInferenceAccelerator)
        pub fn build(self) -> crate::model::ElasticInferenceAccelerator {
            crate::model::ElasticInferenceAccelerator {
                accelerator_health: self.accelerator_health,
                accelerator_type: self.accelerator_type,
                accelerator_id: self.accelerator_id,
                availability_zone: self.availability_zone,
                attached_resource: self.attached_resource,
            }
        }
    }
}
impl ElasticInferenceAccelerator {
    /// Creates a new builder-style object to manufacture [`ElasticInferenceAccelerator`](crate::model::ElasticInferenceAccelerator)
    pub fn builder() -> crate::model::elastic_inference_accelerator::Builder {
        crate::model::elastic_inference_accelerator::Builder::default()
    }
}

/// <p> The health details of an Elastic Inference Accelerator. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ElasticInferenceAcceleratorHealth {
    /// <p> The health status of the Elastic Inference Accelerator. </p>
    pub status: std::option::Option<std::string::String>,
}
impl ElasticInferenceAcceleratorHealth {
    /// <p> The health status of the Elastic Inference Accelerator. </p>
    pub fn status(&self) -> std::option::Option<&str> {
        self.status.as_deref()
    }
}
impl std::fmt::Debug for ElasticInferenceAcceleratorHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ElasticInferenceAcceleratorHealth");
        formatter.field("status", &self.status);
        formatter.finish()
    }
}
/// See [`ElasticInferenceAcceleratorHealth`](crate::model::ElasticInferenceAcceleratorHealth)
pub mod elastic_inference_accelerator_health {

    /// A builder for [`ElasticInferenceAcceleratorHealth`](crate::model::ElasticInferenceAcceleratorHealth)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p> The health status of the Elastic Inference Accelerator. </p>
        pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
            self.status = Some(input.into());
            self
        }
        /// <p> The health status of the Elastic Inference Accelerator. </p>
        pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status = input;
            self
        }
        /// Consumes the builder and constructs a [`ElasticInferenceAcceleratorHealth`](crate::model::ElasticInferenceAcceleratorHealth)
        pub fn build(self) -> crate::model::ElasticInferenceAcceleratorHealth {
            crate::model::ElasticInferenceAcceleratorHealth {
                status: self.status,
            }
        }
    }
}
impl ElasticInferenceAcceleratorHealth {
    /// Creates a new builder-style object to manufacture [`ElasticInferenceAcceleratorHealth`](crate::model::ElasticInferenceAcceleratorHealth)
    pub fn builder() -> crate::model::elastic_inference_accelerator_health::Builder {
        crate::model::elastic_inference_accelerator_health::Builder::default()
    }
}

/// <p> A filter expression for the Elastic Inference Accelerator list. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Filter {
    /// <p> The filter name for the Elastic Inference Accelerator list. It can assume the following values: accelerator-type: the type of Elastic Inference Accelerator to filter for. instance-id: an EC2 instance id to filter for. </p>
    pub name: std::option::Option<std::string::String>,
    /// <p> The values for the filter of the Elastic Inference Accelerator list. </p>
    pub values: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl Filter {
    /// <p> The filter name for the Elastic Inference Accelerator list. It can assume the following values: accelerator-type: the type of Elastic Inference Accelerator to filter for. instance-id: an EC2 instance id to filter for. </p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p> The values for the filter of the Elastic Inference Accelerator list. </p>
    pub fn values(&self) -> std::option::Option<&[std::string::String]> {
        self.values.as_deref()
    }
}
impl std::fmt::Debug for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Filter");
        formatter.field("name", &self.name);
        formatter.field("values", &self.values);
        formatter.finish()
    }
}
/// See [`Filter`](crate::model::Filter)
pub mod filter {

    /// A builder for [`Filter`](crate::model::Filter)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) values: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p> The filter name for the Elastic Inference Accelerator list. It can assume the following values: accelerator-type: the type of Elastic Inference Accelerator to filter for. instance-id: an EC2 instance id to filter for. </p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p> The filter name for the Elastic Inference Accelerator list. It can assume the following values: accelerator-type: the type of Elastic Inference Accelerator to filter for. instance-id: an EC2 instance id to filter for. </p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// Appends an item to `values`.
        ///
        /// To override the contents of this collection use [`set_values`](Self::set_values).
        ///
        /// <p> The values for the filter of the Elastic Inference Accelerator list. </p>
        pub fn values(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.values.unwrap_or_default();
            v.push(input.into());
            self.values = Some(v);
            self
        }
        /// <p> The values for the filter of the Elastic Inference Accelerator list. </p>
        pub fn set_values(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.values = input;
            self
        }
        /// Consumes the builder and constructs a [`Filter`](crate::model::Filter)
        pub fn build(self) -> crate::model::Filter {
            crate::model::Filter {
                name: self.name,
                values: self.values,
            }
        }
    }
}
impl Filter {
    /// Creates a new builder-style object to manufacture [`Filter`](crate::model::Filter)
    pub fn builder() -> crate::model::filter::Builder {
        crate::model::filter::Builder::default()
    }
}

/// <p> The offering for an Elastic Inference Accelerator type. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AcceleratorTypeOffering {
    /// <p> The name of the Elastic Inference Accelerator type. </p>
    pub accelerator_type: std::option::Option<std::string::String>,
    /// <p> The location type for the offering. It can assume the following values: region: defines that the offering is at the regional level. availability-zone: defines that the offering is at the availability zone level. availability-zone-id: defines that the offering is at the availability zone level, defined by the availability zone id. </p>
    pub location_type: std::option::Option<crate::model::LocationType>,
    /// <p> The location for the offering. It will return either the region, availability zone or availability zone id for the offering depending on the locationType value. </p>
    pub location: std::option::Option<std::string::String>,
}
impl AcceleratorTypeOffering {
    /// <p> The name of the Elastic Inference Accelerator type. </p>
    pub fn accelerator_type(&self) -> std::option::Option<&str> {
        self.accelerator_type.as_deref()
    }
    /// <p> The location type for the offering. It can assume the following values: region: defines that the offering is at the regional level. availability-zone: defines that the offering is at the availability zone level. availability-zone-id: defines that the offering is at the availability zone level, defined by the availability zone id. </p>
    pub fn location_type(&self) -> std::option::Option<&crate::model::LocationType> {
        self.location_type.as_ref()
    }
    /// <p> The location for the offering. It will return either the region, availability zone or availability zone id for the offering depending on the locationType value. </p>
    pub fn location(&self) -> std::option::Option<&str> {
        self.location.as_deref()
    }
}
impl std::fmt::Debug for AcceleratorTypeOffering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AcceleratorTypeOffering");
        formatter.field("accelerator_type", &self.accelerator_type);
        formatter.field("location_type", &self.location_type);
        formatter.field("location", &self.location);
        formatter.finish()
    }
}
/// See [`AcceleratorTypeOffering`](crate::model::AcceleratorTypeOffering)
pub mod accelerator_type_offering {

    /// A builder for [`AcceleratorTypeOffering`](crate::model::AcceleratorTypeOffering)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) accelerator_type: std::option::Option<std::string::String>,
        pub(crate) location_type: std::option::Option<crate::model::LocationType>,
        pub(crate) location: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p> The name of the Elastic Inference Accelerator type. </p>
        pub fn accelerator_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.accelerator_type = Some(input.into());
            self
        }
        /// <p> The name of the Elastic Inference Accelerator type. </p>
        pub fn set_accelerator_type(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.accelerator_type = input;
            self
        }
        /// <p> The location type for the offering. It can assume the following values: region: defines that the offering is at the regional level. availability-zone: defines that the offering is at the availability zone level. availability-zone-id: defines that the offering is at the availability zone level, defined by the availability zone id. </p>
        pub fn location_type(mut self, input: crate::model::LocationType) -> Self {
            self.location_type = Some(input);
            self
        }
        /// <p> The location type for the offering. It can assume the following values: region: defines that the offering is at the regional level. availability-zone: defines that the offering is at the availability zone level. availability-zone-id: defines that the offering is at the availability zone level, defined by the availability zone id. </p>
        pub fn set_location_type(
            mut self,
            input: std::option::Option<crate::model::LocationType>,
        ) -> Self {
            self.location_type = input;
            self
        }
        /// <p> The location for the offering. It will return either the region, availability zone or availability zone id for the offering depending on the locationType value. </p>
        pub fn location(mut self, input: impl Into<std::string::String>) -> Self {
            self.location = Some(input.into());
            self
        }
        /// <p> The location for the offering. It will return either the region, availability zone or availability zone id for the offering depending on the locationType value. </p>
        pub fn set_location(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.location = input;
            self
        }
        /// Consumes the builder and constructs a [`AcceleratorTypeOffering`](crate::model::AcceleratorTypeOffering)
        pub fn build(self) -> crate::model::AcceleratorTypeOffering {
            crate::model::AcceleratorTypeOffering {
                accelerator_type: self.accelerator_type,
                location_type: self.location_type,
                location: self.location,
            }
        }
    }
}
impl AcceleratorTypeOffering {
    /// Creates a new builder-style object to manufacture [`AcceleratorTypeOffering`](crate::model::AcceleratorTypeOffering)
    pub fn builder() -> crate::model::accelerator_type_offering::Builder {
        crate::model::accelerator_type_offering::Builder::default()
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
pub enum LocationType {
    #[allow(missing_docs)] // documentation missing in model
    AvailabilityZone,
    #[allow(missing_docs)] // documentation missing in model
    AvailabilityZoneId,
    #[allow(missing_docs)] // documentation missing in model
    Region,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for LocationType {
    fn from(s: &str) -> Self {
        match s {
            "availability-zone" => LocationType::AvailabilityZone,
            "availability-zone-id" => LocationType::AvailabilityZoneId,
            "region" => LocationType::Region,
            other => LocationType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for LocationType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(LocationType::from(s))
    }
}
impl LocationType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            LocationType::AvailabilityZone => "availability-zone",
            LocationType::AvailabilityZoneId => "availability-zone-id",
            LocationType::Region => "region",
            LocationType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["availability-zone", "availability-zone-id", "region"]
    }
}
impl AsRef<str> for LocationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
