// Copyright 2015-2018 Deyan Ginev. See the LICENSE
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Virtual tables/ORM for reports produced by the `CorTeX` backend

#![allow(missing_docs, unused_imports)]
/// Table declaration for the return type for aggregate report queries
table! {
  aggregate_reports (report_name) {
    report_name -> Nullable<Text>,
    task_count -> BigInt,
    message_count -> BigInt,
  }
}

#[derive(Debug, Clone, PartialEq, Eq, QueryableByName)]
/// The return struct of aggregate reports targeting task and log message counts
pub struct AggregateReport {
  /// the category, per `LaTeXML` convention
  pub report_name: Option<String>,
  /// number of tasks with messages under this category (in implied severity - strictly)
  pub task_count: i64,
  /// number of messages under this category (in implied severity - strictly)
  pub message_count: i64,
}

/// Table declaration of the return type for "task details" report queries
table! {
  task_detail_reports (id) {
    id -> BigInt,
    entry -> Text,
    details -> Text,
  }
}

#[derive(Debug, Clone, PartialEq, Eq, QueryableByName)]
/// The return struct of "task details" reports
pub struct TaskDetailReport {
  /// the task id
  pub id: i64,
  /// the task entry path
  pub entry: String,
  /// the details for the queried log (severity, category,what) selection
  pub details: String,
}
