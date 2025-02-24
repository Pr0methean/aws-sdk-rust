// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The search criteria.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct SearchCriteria {
    /// <p>The filters for the search criteria.</p>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::SearchFilter>>,
}
impl SearchCriteria {
    /// <p>The filters for the search criteria.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::SearchFilter] {
        self.filters.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for SearchCriteria {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SearchCriteria");
        formatter.field("filters", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl SearchCriteria {
    /// Creates a new builder-style object to manufacture [`SearchCriteria`](crate::types::SearchCriteria).
    pub fn builder() -> crate::types::builders::SearchCriteriaBuilder {
        crate::types::builders::SearchCriteriaBuilder::default()
    }
}

/// A builder for [`SearchCriteria`](crate::types::SearchCriteria).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct SearchCriteriaBuilder {
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::SearchFilter>>,
}
impl SearchCriteriaBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters for the search criteria.</p>
    pub fn filters(mut self, input: crate::types::SearchFilter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The filters for the search criteria.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SearchFilter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>The filters for the search criteria.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SearchFilter>> {
        &self.filters
    }
    /// Consumes the builder and constructs a [`SearchCriteria`](crate::types::SearchCriteria).
    pub fn build(self) -> crate::types::SearchCriteria {
        crate::types::SearchCriteria { filters: self.filters }
    }
}
impl ::std::fmt::Debug for SearchCriteriaBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SearchCriteriaBuilder");
        formatter.field("filters", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
