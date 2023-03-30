// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A home region control is an object that specifies the home region for an account, with some additional information. It contains a target (always of type <code>ACCOUNT</code>), an ID, and a time at which the home region was set.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct HomeRegionControl  {
    /// <p>A unique identifier that's generated for each home region control. It's always a string that begins with "hrc-" followed by 12 lowercase letters and numbers.</p>
    #[doc(hidden)]
    pub control_id: std::option::Option<std::string::String>,
    /// <p>The AWS Region that's been set as home region. For example, "us-west-2" or "eu-central-1" are valid home regions.</p>
    #[doc(hidden)]
    pub home_region: std::option::Option<std::string::String>,
    /// <p>The target parameter specifies the identifier to which the home region is applied, which is always an <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    #[doc(hidden)]
    pub target: std::option::Option<crate::model::Target>,
    /// <p>A timestamp representing the time when the customer called <code>CreateHomeregionControl</code> and set the home region for the account.</p>
    #[doc(hidden)]
    pub requested_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl HomeRegionControl {
    /// <p>A unique identifier that's generated for each home region control. It's always a string that begins with "hrc-" followed by 12 lowercase letters and numbers.</p>
    pub fn control_id(&self) -> std::option::Option<& str> {
        self.control_id.as_deref()
    }
    /// <p>The AWS Region that's been set as home region. For example, "us-west-2" or "eu-central-1" are valid home regions.</p>
    pub fn home_region(&self) -> std::option::Option<& str> {
        self.home_region.as_deref()
    }
    /// <p>The target parameter specifies the identifier to which the home region is applied, which is always an <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    pub fn target(&self) -> std::option::Option<& crate::model::Target> {
        self.target.as_ref()
    }
    /// <p>A timestamp representing the time when the customer called <code>CreateHomeregionControl</code> and set the home region for the account.</p>
    pub fn requested_time(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.requested_time.as_ref()
    }
}
/// See [`HomeRegionControl`](crate::model::HomeRegionControl).
pub mod home_region_control {
    
    /// A builder for [`HomeRegionControl`](crate::model::HomeRegionControl).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) control_id: std::option::Option<std::string::String>,
        pub(crate) home_region: std::option::Option<std::string::String>,
        pub(crate) target: std::option::Option<crate::model::Target>,
        pub(crate) requested_time: std::option::Option<aws_smithy_types::DateTime>,
    }
    impl Builder {
        /// <p>A unique identifier that's generated for each home region control. It's always a string that begins with "hrc-" followed by 12 lowercase letters and numbers.</p>
        pub fn control_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.control_id = Some(input.into());
            self
        }
        /// <p>A unique identifier that's generated for each home region control. It's always a string that begins with "hrc-" followed by 12 lowercase letters and numbers.</p>
        pub fn set_control_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.control_id = input; self
        }
        /// <p>The AWS Region that's been set as home region. For example, "us-west-2" or "eu-central-1" are valid home regions.</p>
        pub fn home_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.home_region = Some(input.into());
            self
        }
        /// <p>The AWS Region that's been set as home region. For example, "us-west-2" or "eu-central-1" are valid home regions.</p>
        pub fn set_home_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.home_region = input; self
        }
        /// <p>The target parameter specifies the identifier to which the home region is applied, which is always an <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
        pub fn target(mut self, input: crate::model::Target) -> Self {
            self.target = Some(input);
            self
        }
        /// <p>The target parameter specifies the identifier to which the home region is applied, which is always an <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
        pub fn set_target(mut self, input: std::option::Option<crate::model::Target>) -> Self {
            self.target = input; self
        }
        /// <p>A timestamp representing the time when the customer called <code>CreateHomeregionControl</code> and set the home region for the account.</p>
        pub fn requested_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.requested_time = Some(input);
            self
        }
        /// <p>A timestamp representing the time when the customer called <code>CreateHomeregionControl</code> and set the home region for the account.</p>
        pub fn set_requested_time(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.requested_time = input; self
        }
        /// Consumes the builder and constructs a [`HomeRegionControl`](crate::model::HomeRegionControl).
        pub fn build(self) -> crate::model::HomeRegionControl {
            crate::model::HomeRegionControl {
                control_id: self.control_id
                ,
                home_region: self.home_region
                ,
                target: self.target
                ,
                requested_time: self.requested_time
                ,
            }
        }
    }
    
    
}
impl HomeRegionControl {
    /// Creates a new builder-style object to manufacture [`HomeRegionControl`](crate::model::HomeRegionControl).
    pub fn builder() -> crate::model::home_region_control::Builder {
        crate::model::home_region_control::Builder::default()
    }
}

/// <p>The target parameter specifies the identifier to which the home region is applied, which is always an <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Target  {
    /// <p>The target type is always an <code>ACCOUNT</code>.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::model::TargetType>,
    /// <p>The <code>TargetID</code> is a 12-character identifier of the <code>ACCOUNT</code> for which the control was created. (This must be the current account.) </p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
}
impl Target {
    /// <p>The target type is always an <code>ACCOUNT</code>.</p>
    pub fn r#type(&self) -> std::option::Option<& crate::model::TargetType> {
        self.r#type.as_ref()
    }
    /// <p>The <code>TargetID</code> is a 12-character identifier of the <code>ACCOUNT</code> for which the control was created. (This must be the current account.) </p>
    pub fn id(&self) -> std::option::Option<& str> {
        self.id.as_deref()
    }
}
/// See [`Target`](crate::model::Target).
pub mod target {
    
    /// A builder for [`Target`](crate::model::Target).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) r#type: std::option::Option<crate::model::TargetType>,
        pub(crate) id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The target type is always an <code>ACCOUNT</code>.</p>
        pub fn r#type(mut self, input: crate::model::TargetType) -> Self {
            self.r#type = Some(input);
            self
        }
        /// <p>The target type is always an <code>ACCOUNT</code>.</p>
        pub fn set_type(mut self, input: std::option::Option<crate::model::TargetType>) -> Self {
            self.r#type = input; self
        }
        /// <p>The <code>TargetID</code> is a 12-character identifier of the <code>ACCOUNT</code> for which the control was created. (This must be the current account.) </p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>The <code>TargetID</code> is a 12-character identifier of the <code>ACCOUNT</code> for which the control was created. (This must be the current account.) </p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input; self
        }
        /// Consumes the builder and constructs a [`Target`](crate::model::Target).
        pub fn build(self) -> crate::model::Target {
            crate::model::Target {
                r#type: self.r#type
                ,
                id: self.id
                ,
            }
        }
    }
    
    
}
impl Target {
    /// Creates a new builder-style object to manufacture [`Target`](crate::model::Target).
    pub fn builder() -> crate::model::target::Builder {
        crate::model::target::Builder::default()
    }
}

/// When writing a match expression against `TargetType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let targettype = unimplemented!();
/// match targettype {
///     TargetType::Account => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `targettype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `TargetType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `TargetType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `TargetType::NewFeature` is defined.
/// Specifically, when `targettype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `TargetType::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum TargetType {
    #[allow(missing_docs)] // documentation missing in model
    Account,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for TargetType {
                fn from(s: &str) -> Self {
                    match s {
                        "ACCOUNT" => TargetType::Account,
other => TargetType::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for TargetType {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(TargetType::from(s))
                }
            }
impl TargetType {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    TargetType::Account => "ACCOUNT",
    TargetType::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["ACCOUNT"]
                }
            }
impl AsRef<str> for TargetType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

