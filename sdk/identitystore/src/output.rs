// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListUsersOutput {
    /// <p>A list of <code>User</code> objects in the identity store.</p>
    pub users: std::option::Option<std::vec::Vec<crate::model::User>>,
    /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListUsersOutput {
    /// <p>A list of <code>User</code> objects in the identity store.</p>
    pub fn users(&self) -> std::option::Option<&[crate::model::User]> {
        self.users.as_deref()
    }
    /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListUsersOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListUsersOutput");
        formatter.field("users", &self.users);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListUsersOutput`](crate::output::ListUsersOutput)
pub mod list_users_output {

    /// A builder for [`ListUsersOutput`](crate::output::ListUsersOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) users: std::option::Option<std::vec::Vec<crate::model::User>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `users`.
        ///
        /// To override the contents of this collection use [`set_users`](Self::set_users).
        ///
        /// <p>A list of <code>User</code> objects in the identity store.</p>
        pub fn users(mut self, input: crate::model::User) -> Self {
            let mut v = self.users.unwrap_or_default();
            v.push(input);
            self.users = Some(v);
            self
        }
        /// <p>A list of <code>User</code> objects in the identity store.</p>
        pub fn set_users(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::User>>,
        ) -> Self {
            self.users = input;
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListUsersOutput`](crate::output::ListUsersOutput)
        pub fn build(self) -> crate::output::ListUsersOutput {
            crate::output::ListUsersOutput {
                users: self.users,
                next_token: self.next_token,
            }
        }
    }
}
impl ListUsersOutput {
    /// Creates a new builder-style object to manufacture [`ListUsersOutput`](crate::output::ListUsersOutput)
    pub fn builder() -> crate::output::list_users_output::Builder {
        crate::output::list_users_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListGroupsOutput {
    /// <p>A list of <code>Group</code> objects in the identity store.</p>
    pub groups: std::option::Option<std::vec::Vec<crate::model::Group>>,
    /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it1 is used in the API request to search for the next page.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListGroupsOutput {
    /// <p>A list of <code>Group</code> objects in the identity store.</p>
    pub fn groups(&self) -> std::option::Option<&[crate::model::Group]> {
        self.groups.as_deref()
    }
    /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it1 is used in the API request to search for the next page.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListGroupsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListGroupsOutput");
        formatter.field("groups", &self.groups);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListGroupsOutput`](crate::output::ListGroupsOutput)
pub mod list_groups_output {

    /// A builder for [`ListGroupsOutput`](crate::output::ListGroupsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) groups: std::option::Option<std::vec::Vec<crate::model::Group>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `groups`.
        ///
        /// To override the contents of this collection use [`set_groups`](Self::set_groups).
        ///
        /// <p>A list of <code>Group</code> objects in the identity store.</p>
        pub fn groups(mut self, input: crate::model::Group) -> Self {
            let mut v = self.groups.unwrap_or_default();
            v.push(input);
            self.groups = Some(v);
            self
        }
        /// <p>A list of <code>Group</code> objects in the identity store.</p>
        pub fn set_groups(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Group>>,
        ) -> Self {
            self.groups = input;
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it1 is used in the API request to search for the next page.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it1 is used in the API request to search for the next page.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListGroupsOutput`](crate::output::ListGroupsOutput)
        pub fn build(self) -> crate::output::ListGroupsOutput {
            crate::output::ListGroupsOutput {
                groups: self.groups,
                next_token: self.next_token,
            }
        }
    }
}
impl ListGroupsOutput {
    /// Creates a new builder-style object to manufacture [`ListGroupsOutput`](crate::output::ListGroupsOutput)
    pub fn builder() -> crate::output::list_groups_output::Builder {
        crate::output::list_groups_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeUserOutput {
    /// <p>Contains the user’s user name value. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
    pub user_name: std::option::Option<std::string::String>,
    /// <p>The identifier for a user in the identity store.</p>
    pub user_id: std::option::Option<std::string::String>,
}
impl DescribeUserOutput {
    /// <p>Contains the user’s user name value. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
    pub fn user_name(&self) -> std::option::Option<&str> {
        self.user_name.as_deref()
    }
    /// <p>The identifier for a user in the identity store.</p>
    pub fn user_id(&self) -> std::option::Option<&str> {
        self.user_id.as_deref()
    }
}
impl std::fmt::Debug for DescribeUserOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeUserOutput");
        formatter.field("user_name", &"*** Sensitive Data Redacted ***");
        formatter.field("user_id", &self.user_id);
        formatter.finish()
    }
}
/// See [`DescribeUserOutput`](crate::output::DescribeUserOutput)
pub mod describe_user_output {

    /// A builder for [`DescribeUserOutput`](crate::output::DescribeUserOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) user_name: std::option::Option<std::string::String>,
        pub(crate) user_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Contains the user’s user name value. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
        pub fn user_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_name = Some(input.into());
            self
        }
        /// <p>Contains the user’s user name value. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
        pub fn set_user_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_name = input;
            self
        }
        /// <p>The identifier for a user in the identity store.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_id = Some(input.into());
            self
        }
        /// <p>The identifier for a user in the identity store.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeUserOutput`](crate::output::DescribeUserOutput)
        pub fn build(self) -> crate::output::DescribeUserOutput {
            crate::output::DescribeUserOutput {
                user_name: self.user_name,
                user_id: self.user_id,
            }
        }
    }
}
impl DescribeUserOutput {
    /// Creates a new builder-style object to manufacture [`DescribeUserOutput`](crate::output::DescribeUserOutput)
    pub fn builder() -> crate::output::describe_user_output::Builder {
        crate::output::describe_user_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeGroupOutput {
    /// <p>The identifier for a group in the identity store.</p>
    pub group_id: std::option::Option<std::string::String>,
    /// <p>Contains the group’s display name value. The length limit is 1,024 characters. This value can consist of letters, accented characters, symbols, numbers, punctuation, tab, new line, carriage return, space, and nonbreaking space in this attribute. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time that the group is created and stored as an attribute of the group object in the identity store.</p>
    pub display_name: std::option::Option<std::string::String>,
}
impl DescribeGroupOutput {
    /// <p>The identifier for a group in the identity store.</p>
    pub fn group_id(&self) -> std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>Contains the group’s display name value. The length limit is 1,024 characters. This value can consist of letters, accented characters, symbols, numbers, punctuation, tab, new line, carriage return, space, and nonbreaking space in this attribute. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time that the group is created and stored as an attribute of the group object in the identity store.</p>
    pub fn display_name(&self) -> std::option::Option<&str> {
        self.display_name.as_deref()
    }
}
impl std::fmt::Debug for DescribeGroupOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeGroupOutput");
        formatter.field("group_id", &self.group_id);
        formatter.field("display_name", &self.display_name);
        formatter.finish()
    }
}
/// See [`DescribeGroupOutput`](crate::output::DescribeGroupOutput)
pub mod describe_group_output {

    /// A builder for [`DescribeGroupOutput`](crate::output::DescribeGroupOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) group_id: std::option::Option<std::string::String>,
        pub(crate) display_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The identifier for a group in the identity store.</p>
        pub fn group_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.group_id = Some(input.into());
            self
        }
        /// <p>The identifier for a group in the identity store.</p>
        pub fn set_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.group_id = input;
            self
        }
        /// <p>Contains the group’s display name value. The length limit is 1,024 characters. This value can consist of letters, accented characters, symbols, numbers, punctuation, tab, new line, carriage return, space, and nonbreaking space in this attribute. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time that the group is created and stored as an attribute of the group object in the identity store.</p>
        pub fn display_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.display_name = Some(input.into());
            self
        }
        /// <p>Contains the group’s display name value. The length limit is 1,024 characters. This value can consist of letters, accented characters, symbols, numbers, punctuation, tab, new line, carriage return, space, and nonbreaking space in this attribute. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time that the group is created and stored as an attribute of the group object in the identity store.</p>
        pub fn set_display_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.display_name = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeGroupOutput`](crate::output::DescribeGroupOutput)
        pub fn build(self) -> crate::output::DescribeGroupOutput {
            crate::output::DescribeGroupOutput {
                group_id: self.group_id,
                display_name: self.display_name,
            }
        }
    }
}
impl DescribeGroupOutput {
    /// Creates a new builder-style object to manufacture [`DescribeGroupOutput`](crate::output::DescribeGroupOutput)
    pub fn builder() -> crate::output::describe_group_output::Builder {
        crate::output::describe_group_output::Builder::default()
    }
}
