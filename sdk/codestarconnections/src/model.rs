// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The VPC configuration provisioned for the host.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct VpcConfiguration {
    /// <p>The ID of the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
    pub vpc_id: std::option::Option<std::string::String>,
    /// <p>The ID of the subnet or subnets associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
    pub subnet_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The ID of the security group or security groups associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
    pub security_group_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The value of the Transport Layer Security (TLS) certificate associated with the infrastructure where your provider type is installed.</p>
    pub tls_certificate: std::option::Option<std::string::String>,
}
impl VpcConfiguration {
    /// <p>The ID of the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The ID of the subnet or subnets associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
    pub fn subnet_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.subnet_ids.as_deref()
    }
    /// <p>The ID of the security group or security groups associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
    pub fn security_group_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.security_group_ids.as_deref()
    }
    /// <p>The value of the Transport Layer Security (TLS) certificate associated with the infrastructure where your provider type is installed.</p>
    pub fn tls_certificate(&self) -> std::option::Option<&str> {
        self.tls_certificate.as_deref()
    }
}
impl std::fmt::Debug for VpcConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("VpcConfiguration");
        formatter.field("vpc_id", &self.vpc_id);
        formatter.field("subnet_ids", &self.subnet_ids);
        formatter.field("security_group_ids", &self.security_group_ids);
        formatter.field("tls_certificate", &self.tls_certificate);
        formatter.finish()
    }
}
/// See [`VpcConfiguration`](crate::model::VpcConfiguration)
pub mod vpc_configuration {

    /// A builder for [`VpcConfiguration`](crate::model::VpcConfiguration)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) vpc_id: std::option::Option<std::string::String>,
        pub(crate) subnet_ids: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) security_group_ids: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) tls_certificate: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
        pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.vpc_id = Some(input.into());
            self
        }
        /// <p>The ID of the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
        pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.vpc_id = input;
            self
        }
        /// Appends an item to `subnet_ids`.
        ///
        /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
        ///
        /// <p>The ID of the subnet or subnets associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
        pub fn subnet_ids(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.subnet_ids.unwrap_or_default();
            v.push(input.into());
            self.subnet_ids = Some(v);
            self
        }
        /// <p>The ID of the subnet or subnets associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
        pub fn set_subnet_ids(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.subnet_ids = input;
            self
        }
        /// Appends an item to `security_group_ids`.
        ///
        /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
        ///
        /// <p>The ID of the security group or security groups associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
        pub fn security_group_ids(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.security_group_ids.unwrap_or_default();
            v.push(input.into());
            self.security_group_ids = Some(v);
            self
        }
        /// <p>The ID of the security group or security groups associated with the Amazon VPC connected to the infrastructure where your provider type is installed.</p>
        pub fn set_security_group_ids(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.security_group_ids = input;
            self
        }
        /// <p>The value of the Transport Layer Security (TLS) certificate associated with the infrastructure where your provider type is installed.</p>
        pub fn tls_certificate(mut self, input: impl Into<std::string::String>) -> Self {
            self.tls_certificate = Some(input.into());
            self
        }
        /// <p>The value of the Transport Layer Security (TLS) certificate associated with the infrastructure where your provider type is installed.</p>
        pub fn set_tls_certificate(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.tls_certificate = input;
            self
        }
        /// Consumes the builder and constructs a [`VpcConfiguration`](crate::model::VpcConfiguration)
        pub fn build(self) -> crate::model::VpcConfiguration {
            crate::model::VpcConfiguration {
                vpc_id: self.vpc_id,
                subnet_ids: self.subnet_ids,
                security_group_ids: self.security_group_ids,
                tls_certificate: self.tls_certificate,
            }
        }
    }
}
impl VpcConfiguration {
    /// Creates a new builder-style object to manufacture [`VpcConfiguration`](crate::model::VpcConfiguration)
    pub fn builder() -> crate::model::vpc_configuration::Builder {
        crate::model::vpc_configuration::Builder::default()
    }
}

/// <p>A tag is a key-value pair that is used to manage the resource.</p>
/// <p>This tag is available for use by AWS services that support tags.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>The tag's key.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>The tag's value.</p>
    pub value: std::option::Option<std::string::String>,
}
impl Tag {
    /// <p>The tag's key.</p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The tag's value.</p>
    pub fn value(&self) -> std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &self.key);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {

    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The tag's key.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        /// <p>The tag's key.</p>
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>The tag's value.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        /// <p>The tag's value.</p>
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

/// <p>A resource that represents the infrastructure where a third-party provider is installed. The host is used when you create connections to an installed third-party provider type, such as GitHub Enterprise Server. You create one host for all connections to that provider.</p> <note>
/// <p>A host created through the CLI or the SDK is in `PENDING` status by default. You can make its status `AVAILABLE` by setting up the host in the console.</p>
/// </note>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Host {
    /// <p>The name of the host.</p>
    pub name: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the host.</p>
    pub host_arn: std::option::Option<std::string::String>,
    /// <p>The name of the installed provider to be associated with your connection. The host resource represents the infrastructure where your provider type is installed. The valid provider type is GitHub Enterprise Server.</p>
    pub provider_type: std::option::Option<crate::model::ProviderType>,
    /// <p>The endpoint of the infrastructure where your provider type is installed.</p>
    pub provider_endpoint: std::option::Option<std::string::String>,
    /// <p>The VPC configuration provisioned for the host.</p>
    pub vpc_configuration: std::option::Option<crate::model::VpcConfiguration>,
    /// <p>The status of the host, such as PENDING, AVAILABLE, VPC_CONFIG_DELETING, VPC_CONFIG_INITIALIZING, and VPC_CONFIG_FAILED_INITIALIZATION.</p>
    pub status: std::option::Option<std::string::String>,
    /// <p>The status description for the host.</p>
    pub status_message: std::option::Option<std::string::String>,
}
impl Host {
    /// <p>The name of the host.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the host.</p>
    pub fn host_arn(&self) -> std::option::Option<&str> {
        self.host_arn.as_deref()
    }
    /// <p>The name of the installed provider to be associated with your connection. The host resource represents the infrastructure where your provider type is installed. The valid provider type is GitHub Enterprise Server.</p>
    pub fn provider_type(&self) -> std::option::Option<&crate::model::ProviderType> {
        self.provider_type.as_ref()
    }
    /// <p>The endpoint of the infrastructure where your provider type is installed.</p>
    pub fn provider_endpoint(&self) -> std::option::Option<&str> {
        self.provider_endpoint.as_deref()
    }
    /// <p>The VPC configuration provisioned for the host.</p>
    pub fn vpc_configuration(&self) -> std::option::Option<&crate::model::VpcConfiguration> {
        self.vpc_configuration.as_ref()
    }
    /// <p>The status of the host, such as PENDING, AVAILABLE, VPC_CONFIG_DELETING, VPC_CONFIG_INITIALIZING, and VPC_CONFIG_FAILED_INITIALIZATION.</p>
    pub fn status(&self) -> std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The status description for the host.</p>
    pub fn status_message(&self) -> std::option::Option<&str> {
        self.status_message.as_deref()
    }
}
impl std::fmt::Debug for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Host");
        formatter.field("name", &self.name);
        formatter.field("host_arn", &self.host_arn);
        formatter.field("provider_type", &self.provider_type);
        formatter.field("provider_endpoint", &self.provider_endpoint);
        formatter.field("vpc_configuration", &self.vpc_configuration);
        formatter.field("status", &self.status);
        formatter.field("status_message", &self.status_message);
        formatter.finish()
    }
}
/// See [`Host`](crate::model::Host)
pub mod host {

    /// A builder for [`Host`](crate::model::Host)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) host_arn: std::option::Option<std::string::String>,
        pub(crate) provider_type: std::option::Option<crate::model::ProviderType>,
        pub(crate) provider_endpoint: std::option::Option<std::string::String>,
        pub(crate) vpc_configuration: std::option::Option<crate::model::VpcConfiguration>,
        pub(crate) status: std::option::Option<std::string::String>,
        pub(crate) status_message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the host.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name of the host.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the host.</p>
        pub fn host_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.host_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the host.</p>
        pub fn set_host_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.host_arn = input;
            self
        }
        /// <p>The name of the installed provider to be associated with your connection. The host resource represents the infrastructure where your provider type is installed. The valid provider type is GitHub Enterprise Server.</p>
        pub fn provider_type(mut self, input: crate::model::ProviderType) -> Self {
            self.provider_type = Some(input);
            self
        }
        /// <p>The name of the installed provider to be associated with your connection. The host resource represents the infrastructure where your provider type is installed. The valid provider type is GitHub Enterprise Server.</p>
        pub fn set_provider_type(
            mut self,
            input: std::option::Option<crate::model::ProviderType>,
        ) -> Self {
            self.provider_type = input;
            self
        }
        /// <p>The endpoint of the infrastructure where your provider type is installed.</p>
        pub fn provider_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
            self.provider_endpoint = Some(input.into());
            self
        }
        /// <p>The endpoint of the infrastructure where your provider type is installed.</p>
        pub fn set_provider_endpoint(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.provider_endpoint = input;
            self
        }
        /// <p>The VPC configuration provisioned for the host.</p>
        pub fn vpc_configuration(mut self, input: crate::model::VpcConfiguration) -> Self {
            self.vpc_configuration = Some(input);
            self
        }
        /// <p>The VPC configuration provisioned for the host.</p>
        pub fn set_vpc_configuration(
            mut self,
            input: std::option::Option<crate::model::VpcConfiguration>,
        ) -> Self {
            self.vpc_configuration = input;
            self
        }
        /// <p>The status of the host, such as PENDING, AVAILABLE, VPC_CONFIG_DELETING, VPC_CONFIG_INITIALIZING, and VPC_CONFIG_FAILED_INITIALIZATION.</p>
        pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
            self.status = Some(input.into());
            self
        }
        /// <p>The status of the host, such as PENDING, AVAILABLE, VPC_CONFIG_DELETING, VPC_CONFIG_INITIALIZING, and VPC_CONFIG_FAILED_INITIALIZATION.</p>
        pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status = input;
            self
        }
        /// <p>The status description for the host.</p>
        pub fn status_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.status_message = Some(input.into());
            self
        }
        /// <p>The status description for the host.</p>
        pub fn set_status_message(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.status_message = input;
            self
        }
        /// Consumes the builder and constructs a [`Host`](crate::model::Host)
        pub fn build(self) -> crate::model::Host {
            crate::model::Host {
                name: self.name,
                host_arn: self.host_arn,
                provider_type: self.provider_type,
                provider_endpoint: self.provider_endpoint,
                vpc_configuration: self.vpc_configuration,
                status: self.status,
                status_message: self.status_message,
            }
        }
    }
}
impl Host {
    /// Creates a new builder-style object to manufacture [`Host`](crate::model::Host)
    pub fn builder() -> crate::model::host::Builder {
        crate::model::host::Builder::default()
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
pub enum ProviderType {
    #[allow(missing_docs)] // documentation missing in model
    Bitbucket,
    #[allow(missing_docs)] // documentation missing in model
    Github,
    #[allow(missing_docs)] // documentation missing in model
    GithubEnterpriseServer,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ProviderType {
    fn from(s: &str) -> Self {
        match s {
            "Bitbucket" => ProviderType::Bitbucket,
            "GitHub" => ProviderType::Github,
            "GitHubEnterpriseServer" => ProviderType::GithubEnterpriseServer,
            other => ProviderType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ProviderType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ProviderType::from(s))
    }
}
impl ProviderType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ProviderType::Bitbucket => "Bitbucket",
            ProviderType::Github => "GitHub",
            ProviderType::GithubEnterpriseServer => "GitHubEnterpriseServer",
            ProviderType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["Bitbucket", "GitHub", "GitHubEnterpriseServer"]
    }
}
impl AsRef<str> for ProviderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>A resource that is used to connect third-party source providers with services like AWS CodePipeline.</p>
/// <p>Note: A connection created through CloudFormation, the CLI, or the SDK is in `PENDING` status by default. You can make its status `AVAILABLE` by updating the connection in the console.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Connection {
    /// <p>The name of the connection. Connection names must be unique in an AWS user account.</p>
    pub connection_name: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the connection. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note>
    /// <p>The ARN is never reused if the connection is deleted.</p>
    /// </note>
    pub connection_arn: std::option::Option<std::string::String>,
    /// <p>The name of the external provider where your third-party code repository is configured.</p>
    pub provider_type: std::option::Option<crate::model::ProviderType>,
    /// <p>The identifier of the external provider where your third-party code repository is configured. For Bitbucket, this is the account ID of the owner of the Bitbucket repository.</p>
    pub owner_account_id: std::option::Option<std::string::String>,
    /// <p>The current status of the connection. </p>
    pub connection_status: std::option::Option<crate::model::ConnectionStatus>,
    /// <p>The Amazon Resource Name (ARN) of the host associated with the connection.</p>
    pub host_arn: std::option::Option<std::string::String>,
}
impl Connection {
    /// <p>The name of the connection. Connection names must be unique in an AWS user account.</p>
    pub fn connection_name(&self) -> std::option::Option<&str> {
        self.connection_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the connection. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note>
    /// <p>The ARN is never reused if the connection is deleted.</p>
    /// </note>
    pub fn connection_arn(&self) -> std::option::Option<&str> {
        self.connection_arn.as_deref()
    }
    /// <p>The name of the external provider where your third-party code repository is configured.</p>
    pub fn provider_type(&self) -> std::option::Option<&crate::model::ProviderType> {
        self.provider_type.as_ref()
    }
    /// <p>The identifier of the external provider where your third-party code repository is configured. For Bitbucket, this is the account ID of the owner of the Bitbucket repository.</p>
    pub fn owner_account_id(&self) -> std::option::Option<&str> {
        self.owner_account_id.as_deref()
    }
    /// <p>The current status of the connection. </p>
    pub fn connection_status(&self) -> std::option::Option<&crate::model::ConnectionStatus> {
        self.connection_status.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the host associated with the connection.</p>
    pub fn host_arn(&self) -> std::option::Option<&str> {
        self.host_arn.as_deref()
    }
}
impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Connection");
        formatter.field("connection_name", &self.connection_name);
        formatter.field("connection_arn", &self.connection_arn);
        formatter.field("provider_type", &self.provider_type);
        formatter.field("owner_account_id", &self.owner_account_id);
        formatter.field("connection_status", &self.connection_status);
        formatter.field("host_arn", &self.host_arn);
        formatter.finish()
    }
}
/// See [`Connection`](crate::model::Connection)
pub mod connection {

    /// A builder for [`Connection`](crate::model::Connection)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) connection_name: std::option::Option<std::string::String>,
        pub(crate) connection_arn: std::option::Option<std::string::String>,
        pub(crate) provider_type: std::option::Option<crate::model::ProviderType>,
        pub(crate) owner_account_id: std::option::Option<std::string::String>,
        pub(crate) connection_status: std::option::Option<crate::model::ConnectionStatus>,
        pub(crate) host_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the connection. Connection names must be unique in an AWS user account.</p>
        pub fn connection_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.connection_name = Some(input.into());
            self
        }
        /// <p>The name of the connection. Connection names must be unique in an AWS user account.</p>
        pub fn set_connection_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.connection_name = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the connection. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note>
        /// <p>The ARN is never reused if the connection is deleted.</p>
        /// </note>
        pub fn connection_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.connection_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the connection. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note>
        /// <p>The ARN is never reused if the connection is deleted.</p>
        /// </note>
        pub fn set_connection_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.connection_arn = input;
            self
        }
        /// <p>The name of the external provider where your third-party code repository is configured.</p>
        pub fn provider_type(mut self, input: crate::model::ProviderType) -> Self {
            self.provider_type = Some(input);
            self
        }
        /// <p>The name of the external provider where your third-party code repository is configured.</p>
        pub fn set_provider_type(
            mut self,
            input: std::option::Option<crate::model::ProviderType>,
        ) -> Self {
            self.provider_type = input;
            self
        }
        /// <p>The identifier of the external provider where your third-party code repository is configured. For Bitbucket, this is the account ID of the owner of the Bitbucket repository.</p>
        pub fn owner_account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.owner_account_id = Some(input.into());
            self
        }
        /// <p>The identifier of the external provider where your third-party code repository is configured. For Bitbucket, this is the account ID of the owner of the Bitbucket repository.</p>
        pub fn set_owner_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.owner_account_id = input;
            self
        }
        /// <p>The current status of the connection. </p>
        pub fn connection_status(mut self, input: crate::model::ConnectionStatus) -> Self {
            self.connection_status = Some(input);
            self
        }
        /// <p>The current status of the connection. </p>
        pub fn set_connection_status(
            mut self,
            input: std::option::Option<crate::model::ConnectionStatus>,
        ) -> Self {
            self.connection_status = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the host associated with the connection.</p>
        pub fn host_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.host_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the host associated with the connection.</p>
        pub fn set_host_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.host_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`Connection`](crate::model::Connection)
        pub fn build(self) -> crate::model::Connection {
            crate::model::Connection {
                connection_name: self.connection_name,
                connection_arn: self.connection_arn,
                provider_type: self.provider_type,
                owner_account_id: self.owner_account_id,
                connection_status: self.connection_status,
                host_arn: self.host_arn,
            }
        }
    }
}
impl Connection {
    /// Creates a new builder-style object to manufacture [`Connection`](crate::model::Connection)
    pub fn builder() -> crate::model::connection::Builder {
        crate::model::connection::Builder::default()
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
pub enum ConnectionStatus {
    #[allow(missing_docs)] // documentation missing in model
    Available,
    #[allow(missing_docs)] // documentation missing in model
    Error,
    #[allow(missing_docs)] // documentation missing in model
    Pending,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ConnectionStatus {
    fn from(s: &str) -> Self {
        match s {
            "AVAILABLE" => ConnectionStatus::Available,
            "ERROR" => ConnectionStatus::Error,
            "PENDING" => ConnectionStatus::Pending,
            other => ConnectionStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ConnectionStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ConnectionStatus::from(s))
    }
}
impl ConnectionStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ConnectionStatus::Available => "AVAILABLE",
            ConnectionStatus::Error => "ERROR",
            ConnectionStatus::Pending => "PENDING",
            ConnectionStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["AVAILABLE", "ERROR", "PENDING"]
    }
}
impl AsRef<str> for ConnectionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
