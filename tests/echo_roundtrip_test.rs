// Copyright 2015-2016 Deyan Ginev. See the LICENSE
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.
extern crate cortex;
extern crate diesel;
extern crate pericortex;

use std::thread;
use cortex::backend;
use cortex::backend::TEST_DB_ADDRESS;
use cortex::models::{Corpus, NewCorpus, Service, NewService, NewTask};
use cortex::helpers::TaskStatus;
use cortex::manager::TaskManager;
use pericortex::worker::{EchoWorker, Worker};
use cortex::importer::Importer;

use diesel::delete;
use diesel::prelude::*;
use cortex::schema::{corpora, services, tasks};

#[test]
fn mock_round_trip() {
  // Initialize a corpus, import a single task, and enable a service on it
  let job_limit: Option<usize> = Some(1);
  let test_backend = backend::testdb();
  // assert!(test_backend.setup_task_tables().is_ok());
  let corpus_name = "mock round-trip corpus";
  // Clean slate
  let clean_slate_result = delete(corpora::table)
    .filter(corpora::name.eq(corpus_name))
    .execute(&test_backend.connection);
  assert!(clean_slate_result.is_ok());

  let add_corpus_result = test_backend.add(&NewCorpus {
    name: corpus_name.to_string(),
    path: "tests/data/".to_string(),
    complex: true,
  });
  assert!(add_corpus_result.is_ok());
  let corpus_result = Corpus::find_by_name(corpus_name, &test_backend.connection);
  assert!(corpus_result.is_ok());
  let mock_corpus = corpus_result.unwrap();

  let service_name = "echo_service";
  let mut abs_path = Importer::cwd();
  abs_path.push("tests/data/1508.01222/1508.01222.zip");
  let abs_entry = abs_path.to_str().unwrap().to_string();

  // clean slate
  let service_clean_slate = delete(services::table)
    .filter(services::name.eq(service_name))
    .execute(&test_backend.connection);
  assert!(service_clean_slate.is_ok());
  let tasks_clean_slate = delete(tasks::table)
    .filter(tasks::entry.eq(&abs_entry))
    .execute(&test_backend.connection);
  assert!(tasks_clean_slate.is_ok());

  let add_service_result = test_backend.add(&NewService {
    name: service_name.to_string(),
    version: 0.1,
    inputformat: "tex".to_string(),
    outputformat: "tex".to_string(),
    inputconverter: Some("import".to_string()),
    complex: true,
  });
  assert!(add_service_result.is_ok());
  let service_result = Service::find_by_name(service_name, &test_backend.connection);
  assert!(service_result.is_ok());
  let echo_service = service_result.unwrap();

  let import_task_result = test_backend.add(&NewTask {
    entry: abs_entry.clone(),
    service_id: 2, // Import service always has id 2
    corpus_id: mock_corpus.id,
    status: TaskStatus::NoProblem.raw(),
  });
  assert!(import_task_result.is_ok());

  let add_echo_task_result = test_backend.add(&NewTask {
    entry: abs_entry.clone(),
    service_id: echo_service.id,
    corpus_id: mock_corpus.id,
    status: TaskStatus::TODO.raw(),
  });
  assert!(add_echo_task_result.is_ok());

  // Start up a ventilator/sink pair
  let manager_thread = thread::spawn(move || {
    let manager = TaskManager {
      source_port: 5555,
      result_port: 5556,
      queue_size: 100_000,
      message_size: 100,
      backend_address: TEST_DB_ADDRESS.to_string(),
    };
    assert!(manager.start(job_limit).is_ok());
  });

  // Start up an echo worker
  let worker = EchoWorker::default();
  // Perform a single echo task
  assert!(worker.start(job_limit).is_ok());
  assert!(manager_thread.join().is_ok());
  // TODO: Check round-trip success
}
