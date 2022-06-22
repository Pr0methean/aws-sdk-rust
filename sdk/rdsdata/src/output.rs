// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The response elements represent the output of a request to perform a rollback of a transaction.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RollbackTransactionOutput {
    /// <p>The status of the rollback operation.</p>
    pub transaction_status: std::option::Option<std::string::String>,
}
impl RollbackTransactionOutput {
    /// <p>The status of the rollback operation.</p>
    pub fn transaction_status(&self) -> std::option::Option<&str> {
        self.transaction_status.as_deref()
    }
}
impl std::fmt::Debug for RollbackTransactionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RollbackTransactionOutput");
        formatter.field("transaction_status", &self.transaction_status);
        formatter.finish()
    }
}
/// See [`RollbackTransactionOutput`](crate::output::RollbackTransactionOutput)
pub mod rollback_transaction_output {

    /// A builder for [`RollbackTransactionOutput`](crate::output::RollbackTransactionOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) transaction_status: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The status of the rollback operation.</p>
        pub fn transaction_status(mut self, input: impl Into<std::string::String>) -> Self {
            self.transaction_status = Some(input.into());
            self
        }
        /// <p>The status of the rollback operation.</p>
        pub fn set_transaction_status(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.transaction_status = input;
            self
        }
        /// Consumes the builder and constructs a [`RollbackTransactionOutput`](crate::output::RollbackTransactionOutput)
        pub fn build(self) -> crate::output::RollbackTransactionOutput {
            crate::output::RollbackTransactionOutput {
                transaction_status: self.transaction_status,
            }
        }
    }
}
impl RollbackTransactionOutput {
    /// Creates a new builder-style object to manufacture [`RollbackTransactionOutput`](crate::output::RollbackTransactionOutput)
    pub fn builder() -> crate::output::rollback_transaction_output::Builder {
        crate::output::rollback_transaction_output::Builder::default()
    }
}

/// <p>The response elements represent the output of a request to run a SQL statement against a database.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ExecuteStatementOutput {
    /// <p>The records returned by the SQL statement. This field is blank if the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
    pub records: std::option::Option<std::vec::Vec<std::vec::Vec<crate::model::Field>>>,
    /// <p>Metadata for the columns included in the results. This field is blank if the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
    pub column_metadata: std::option::Option<std::vec::Vec<crate::model::ColumnMetadata>>,
    /// <p>The number of records updated by the request.</p>
    pub number_of_records_updated: i64,
    /// <p>Values for fields generated during a DML request.</p> <note>
    /// <p>The <code>generatedFields</code> data isn't supported by Aurora PostgreSQL. To get the values of generated fields, use the <code>RETURNING</code> clause. For more information, see <a href="https://www.postgresql.org/docs/10/dml-returning.html">Returning Data From Modified Rows</a> in the PostgreSQL documentation.</p>
    /// </note>
    pub generated_fields: std::option::Option<std::vec::Vec<crate::model::Field>>,
    /// <p>A string value that represents the result set of a <code>SELECT</code> statement in JSON format. This value is only present when the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
    /// <p>The size limit for this field is currently 10 MB. If the JSON-formatted string representing the result set requires more than 10 MB, the call returns an error.</p>
    pub formatted_records: std::option::Option<std::string::String>,
}
impl ExecuteStatementOutput {
    /// <p>The records returned by the SQL statement. This field is blank if the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
    pub fn records(&self) -> std::option::Option<&[std::vec::Vec<crate::model::Field>]> {
        self.records.as_deref()
    }
    /// <p>Metadata for the columns included in the results. This field is blank if the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
    pub fn column_metadata(&self) -> std::option::Option<&[crate::model::ColumnMetadata]> {
        self.column_metadata.as_deref()
    }
    /// <p>The number of records updated by the request.</p>
    pub fn number_of_records_updated(&self) -> i64 {
        self.number_of_records_updated
    }
    /// <p>Values for fields generated during a DML request.</p> <note>
    /// <p>The <code>generatedFields</code> data isn't supported by Aurora PostgreSQL. To get the values of generated fields, use the <code>RETURNING</code> clause. For more information, see <a href="https://www.postgresql.org/docs/10/dml-returning.html">Returning Data From Modified Rows</a> in the PostgreSQL documentation.</p>
    /// </note>
    pub fn generated_fields(&self) -> std::option::Option<&[crate::model::Field]> {
        self.generated_fields.as_deref()
    }
    /// <p>A string value that represents the result set of a <code>SELECT</code> statement in JSON format. This value is only present when the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
    /// <p>The size limit for this field is currently 10 MB. If the JSON-formatted string representing the result set requires more than 10 MB, the call returns an error.</p>
    pub fn formatted_records(&self) -> std::option::Option<&str> {
        self.formatted_records.as_deref()
    }
}
impl std::fmt::Debug for ExecuteStatementOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExecuteStatementOutput");
        formatter.field("records", &self.records);
        formatter.field("column_metadata", &self.column_metadata);
        formatter.field("number_of_records_updated", &self.number_of_records_updated);
        formatter.field("generated_fields", &self.generated_fields);
        formatter.field("formatted_records", &self.formatted_records);
        formatter.finish()
    }
}
/// See [`ExecuteStatementOutput`](crate::output::ExecuteStatementOutput)
pub mod execute_statement_output {

    /// A builder for [`ExecuteStatementOutput`](crate::output::ExecuteStatementOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) records: std::option::Option<std::vec::Vec<std::vec::Vec<crate::model::Field>>>,
        pub(crate) column_metadata:
            std::option::Option<std::vec::Vec<crate::model::ColumnMetadata>>,
        pub(crate) number_of_records_updated: std::option::Option<i64>,
        pub(crate) generated_fields: std::option::Option<std::vec::Vec<crate::model::Field>>,
        pub(crate) formatted_records: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `records`.
        ///
        /// To override the contents of this collection use [`set_records`](Self::set_records).
        ///
        /// <p>The records returned by the SQL statement. This field is blank if the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
        pub fn records(mut self, input: std::vec::Vec<crate::model::Field>) -> Self {
            let mut v = self.records.unwrap_or_default();
            v.push(input);
            self.records = Some(v);
            self
        }
        /// <p>The records returned by the SQL statement. This field is blank if the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
        pub fn set_records(
            mut self,
            input: std::option::Option<std::vec::Vec<std::vec::Vec<crate::model::Field>>>,
        ) -> Self {
            self.records = input;
            self
        }
        /// Appends an item to `column_metadata`.
        ///
        /// To override the contents of this collection use [`set_column_metadata`](Self::set_column_metadata).
        ///
        /// <p>Metadata for the columns included in the results. This field is blank if the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
        pub fn column_metadata(mut self, input: crate::model::ColumnMetadata) -> Self {
            let mut v = self.column_metadata.unwrap_or_default();
            v.push(input);
            self.column_metadata = Some(v);
            self
        }
        /// <p>Metadata for the columns included in the results. This field is blank if the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
        pub fn set_column_metadata(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ColumnMetadata>>,
        ) -> Self {
            self.column_metadata = input;
            self
        }
        /// <p>The number of records updated by the request.</p>
        pub fn number_of_records_updated(mut self, input: i64) -> Self {
            self.number_of_records_updated = Some(input);
            self
        }
        /// <p>The number of records updated by the request.</p>
        pub fn set_number_of_records_updated(mut self, input: std::option::Option<i64>) -> Self {
            self.number_of_records_updated = input;
            self
        }
        /// Appends an item to `generated_fields`.
        ///
        /// To override the contents of this collection use [`set_generated_fields`](Self::set_generated_fields).
        ///
        /// <p>Values for fields generated during a DML request.</p> <note>
        /// <p>The <code>generatedFields</code> data isn't supported by Aurora PostgreSQL. To get the values of generated fields, use the <code>RETURNING</code> clause. For more information, see <a href="https://www.postgresql.org/docs/10/dml-returning.html">Returning Data From Modified Rows</a> in the PostgreSQL documentation.</p>
        /// </note>
        pub fn generated_fields(mut self, input: crate::model::Field) -> Self {
            let mut v = self.generated_fields.unwrap_or_default();
            v.push(input);
            self.generated_fields = Some(v);
            self
        }
        /// <p>Values for fields generated during a DML request.</p> <note>
        /// <p>The <code>generatedFields</code> data isn't supported by Aurora PostgreSQL. To get the values of generated fields, use the <code>RETURNING</code> clause. For more information, see <a href="https://www.postgresql.org/docs/10/dml-returning.html">Returning Data From Modified Rows</a> in the PostgreSQL documentation.</p>
        /// </note>
        pub fn set_generated_fields(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Field>>,
        ) -> Self {
            self.generated_fields = input;
            self
        }
        /// <p>A string value that represents the result set of a <code>SELECT</code> statement in JSON format. This value is only present when the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
        /// <p>The size limit for this field is currently 10 MB. If the JSON-formatted string representing the result set requires more than 10 MB, the call returns an error.</p>
        pub fn formatted_records(mut self, input: impl Into<std::string::String>) -> Self {
            self.formatted_records = Some(input.into());
            self
        }
        /// <p>A string value that represents the result set of a <code>SELECT</code> statement in JSON format. This value is only present when the <code>formatRecordsAs</code> parameter is set to <code>JSON</code>.</p>
        /// <p>The size limit for this field is currently 10 MB. If the JSON-formatted string representing the result set requires more than 10 MB, the call returns an error.</p>
        pub fn set_formatted_records(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.formatted_records = input;
            self
        }
        /// Consumes the builder and constructs a [`ExecuteStatementOutput`](crate::output::ExecuteStatementOutput)
        pub fn build(self) -> crate::output::ExecuteStatementOutput {
            crate::output::ExecuteStatementOutput {
                records: self.records,
                column_metadata: self.column_metadata,
                number_of_records_updated: self.number_of_records_updated.unwrap_or_default(),
                generated_fields: self.generated_fields,
                formatted_records: self.formatted_records,
            }
        }
    }
}
impl ExecuteStatementOutput {
    /// Creates a new builder-style object to manufacture [`ExecuteStatementOutput`](crate::output::ExecuteStatementOutput)
    pub fn builder() -> crate::output::execute_statement_output::Builder {
        crate::output::execute_statement_output::Builder::default()
    }
}

/// <p>The response elements represent the output of a request to run one or more SQL statements.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ExecuteSqlOutput {
    /// <p>The results of the SQL statement or statements.</p>
    pub sql_statement_results: std::option::Option<std::vec::Vec<crate::model::SqlStatementResult>>,
}
impl ExecuteSqlOutput {
    /// <p>The results of the SQL statement or statements.</p>
    pub fn sql_statement_results(
        &self,
    ) -> std::option::Option<&[crate::model::SqlStatementResult]> {
        self.sql_statement_results.as_deref()
    }
}
impl std::fmt::Debug for ExecuteSqlOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExecuteSqlOutput");
        formatter.field("sql_statement_results", &self.sql_statement_results);
        formatter.finish()
    }
}
/// See [`ExecuteSqlOutput`](crate::output::ExecuteSqlOutput)
pub mod execute_sql_output {

    /// A builder for [`ExecuteSqlOutput`](crate::output::ExecuteSqlOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) sql_statement_results:
            std::option::Option<std::vec::Vec<crate::model::SqlStatementResult>>,
    }
    impl Builder {
        /// Appends an item to `sql_statement_results`.
        ///
        /// To override the contents of this collection use [`set_sql_statement_results`](Self::set_sql_statement_results).
        ///
        /// <p>The results of the SQL statement or statements.</p>
        pub fn sql_statement_results(mut self, input: crate::model::SqlStatementResult) -> Self {
            let mut v = self.sql_statement_results.unwrap_or_default();
            v.push(input);
            self.sql_statement_results = Some(v);
            self
        }
        /// <p>The results of the SQL statement or statements.</p>
        pub fn set_sql_statement_results(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SqlStatementResult>>,
        ) -> Self {
            self.sql_statement_results = input;
            self
        }
        /// Consumes the builder and constructs a [`ExecuteSqlOutput`](crate::output::ExecuteSqlOutput)
        pub fn build(self) -> crate::output::ExecuteSqlOutput {
            crate::output::ExecuteSqlOutput {
                sql_statement_results: self.sql_statement_results,
            }
        }
    }
}
impl ExecuteSqlOutput {
    /// Creates a new builder-style object to manufacture [`ExecuteSqlOutput`](crate::output::ExecuteSqlOutput)
    pub fn builder() -> crate::output::execute_sql_output::Builder {
        crate::output::execute_sql_output::Builder::default()
    }
}

/// <p>The response elements represent the output of a commit transaction request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CommitTransactionOutput {
    /// <p>The status of the commit operation.</p>
    pub transaction_status: std::option::Option<std::string::String>,
}
impl CommitTransactionOutput {
    /// <p>The status of the commit operation.</p>
    pub fn transaction_status(&self) -> std::option::Option<&str> {
        self.transaction_status.as_deref()
    }
}
impl std::fmt::Debug for CommitTransactionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CommitTransactionOutput");
        formatter.field("transaction_status", &self.transaction_status);
        formatter.finish()
    }
}
/// See [`CommitTransactionOutput`](crate::output::CommitTransactionOutput)
pub mod commit_transaction_output {

    /// A builder for [`CommitTransactionOutput`](crate::output::CommitTransactionOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) transaction_status: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The status of the commit operation.</p>
        pub fn transaction_status(mut self, input: impl Into<std::string::String>) -> Self {
            self.transaction_status = Some(input.into());
            self
        }
        /// <p>The status of the commit operation.</p>
        pub fn set_transaction_status(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.transaction_status = input;
            self
        }
        /// Consumes the builder and constructs a [`CommitTransactionOutput`](crate::output::CommitTransactionOutput)
        pub fn build(self) -> crate::output::CommitTransactionOutput {
            crate::output::CommitTransactionOutput {
                transaction_status: self.transaction_status,
            }
        }
    }
}
impl CommitTransactionOutput {
    /// Creates a new builder-style object to manufacture [`CommitTransactionOutput`](crate::output::CommitTransactionOutput)
    pub fn builder() -> crate::output::commit_transaction_output::Builder {
        crate::output::commit_transaction_output::Builder::default()
    }
}

/// <p>The response elements represent the output of a request to start a SQL transaction.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BeginTransactionOutput {
    /// <p>The transaction ID of the transaction started by the call.</p>
    pub transaction_id: std::option::Option<std::string::String>,
}
impl BeginTransactionOutput {
    /// <p>The transaction ID of the transaction started by the call.</p>
    pub fn transaction_id(&self) -> std::option::Option<&str> {
        self.transaction_id.as_deref()
    }
}
impl std::fmt::Debug for BeginTransactionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BeginTransactionOutput");
        formatter.field("transaction_id", &self.transaction_id);
        formatter.finish()
    }
}
/// See [`BeginTransactionOutput`](crate::output::BeginTransactionOutput)
pub mod begin_transaction_output {

    /// A builder for [`BeginTransactionOutput`](crate::output::BeginTransactionOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) transaction_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The transaction ID of the transaction started by the call.</p>
        pub fn transaction_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.transaction_id = Some(input.into());
            self
        }
        /// <p>The transaction ID of the transaction started by the call.</p>
        pub fn set_transaction_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.transaction_id = input;
            self
        }
        /// Consumes the builder and constructs a [`BeginTransactionOutput`](crate::output::BeginTransactionOutput)
        pub fn build(self) -> crate::output::BeginTransactionOutput {
            crate::output::BeginTransactionOutput {
                transaction_id: self.transaction_id,
            }
        }
    }
}
impl BeginTransactionOutput {
    /// Creates a new builder-style object to manufacture [`BeginTransactionOutput`](crate::output::BeginTransactionOutput)
    pub fn builder() -> crate::output::begin_transaction_output::Builder {
        crate::output::begin_transaction_output::Builder::default()
    }
}

/// <p>The response elements represent the output of a SQL statement over an array of data.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BatchExecuteStatementOutput {
    /// <p>The execution results of each batch entry.</p>
    pub update_results: std::option::Option<std::vec::Vec<crate::model::UpdateResult>>,
}
impl BatchExecuteStatementOutput {
    /// <p>The execution results of each batch entry.</p>
    pub fn update_results(&self) -> std::option::Option<&[crate::model::UpdateResult]> {
        self.update_results.as_deref()
    }
}
impl std::fmt::Debug for BatchExecuteStatementOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BatchExecuteStatementOutput");
        formatter.field("update_results", &self.update_results);
        formatter.finish()
    }
}
/// See [`BatchExecuteStatementOutput`](crate::output::BatchExecuteStatementOutput)
pub mod batch_execute_statement_output {

    /// A builder for [`BatchExecuteStatementOutput`](crate::output::BatchExecuteStatementOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) update_results: std::option::Option<std::vec::Vec<crate::model::UpdateResult>>,
    }
    impl Builder {
        /// Appends an item to `update_results`.
        ///
        /// To override the contents of this collection use [`set_update_results`](Self::set_update_results).
        ///
        /// <p>The execution results of each batch entry.</p>
        pub fn update_results(mut self, input: crate::model::UpdateResult) -> Self {
            let mut v = self.update_results.unwrap_or_default();
            v.push(input);
            self.update_results = Some(v);
            self
        }
        /// <p>The execution results of each batch entry.</p>
        pub fn set_update_results(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::UpdateResult>>,
        ) -> Self {
            self.update_results = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchExecuteStatementOutput`](crate::output::BatchExecuteStatementOutput)
        pub fn build(self) -> crate::output::BatchExecuteStatementOutput {
            crate::output::BatchExecuteStatementOutput {
                update_results: self.update_results,
            }
        }
    }
}
impl BatchExecuteStatementOutput {
    /// Creates a new builder-style object to manufacture [`BatchExecuteStatementOutput`](crate::output::BatchExecuteStatementOutput)
    pub fn builder() -> crate::output::batch_execute_statement_output::Builder {
        crate::output::batch_execute_statement_output::Builder::default()
    }
}
