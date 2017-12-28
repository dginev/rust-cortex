// Copyright 2015-2018 Deyan Ginev. See the LICENSE
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

///! Import a new corpus into `CorTeX` from the command line.
///! Example run: $ ./target/release/examples/tex_to_html_import /data/arxmliv/ arXMLiv

extern crate cortex;
extern crate pericortex;
extern crate rustc_serialize;

// use std::collections::HashMap;
// use std::path::Path;
// use std::fs;
use std::env;
// use std::io::Read;
// use std::io::Error;

use std::thread;
use std::time::Duration;
use cortex::backend::{Backend, DEFAULT_DB_ADDRESS};
use cortex::models::{NewTask, Service, NewService};
use cortex::helpers::TaskStatus;
use cortex::manager::TaskManager;
use cortex::worker::InitWorker;
use pericortex::worker::Worker;

fn main() {
  let job_limit: Option<usize> = Some(1);
  let mut input_args = env::args();
  let _ = input_args.next();
  let mut corpus_path = match input_args.next() {
    Some(path) => path,
    None => "/arXMLiv/modern".to_string(),
  };
  if let Some(c) = corpus_path.pop() {
    if c != '/' {
      corpus_path.push(c);
    }
  }
  corpus_path.push('/');
  println!("-- Importing corpus at {:?} ...", &corpus_path);
  let backend = Backend::default();

  backend
    .add(&NewTask {
      entry: corpus_path.clone(),
      service_id: 1, // Init service always has id 1
      corpus_id: 1,
      status: TaskStatus::TODO.raw(),
    })
    .unwrap();

  // Let us thread out a ventilator on a special port
  // Start up a ventilator/sink pair
  thread::spawn(move || {
    let manager = TaskManager {
      source_port: 5757,
      result_port: 5758,
      queue_size: 100000,
      message_size: 100,
      backend_address: DEFAULT_DB_ADDRESS.to_string(),
    };
    assert!(manager.start(job_limit).is_ok());
  });

  // Start up an init worker
  let worker = InitWorker {
    service: "init".to_string(),
    version: 0.1,
    message_size: 100000,
    source: "tcp://localhost:5757".to_string(),
    sink: "tcp://localhost:5758".to_string(),
    backend_address: DEFAULT_DB_ADDRESS.to_string(),
  };
  // Perform a single echo task
  assert!(worker.start(job_limit).is_ok());
  // Wait for the final finisher to persist to DB
  thread::sleep(Duration::new(2, 0)); // TODO: Can this be deterministic? Join?

  // Then add a TeX-to-HTML service on this corpus.
  let service_name = "tex_to_html";
  let new_tex_to_html_service = NewService {
    name: service_name.to_string(),
    version: 0.1,
    inputformat: "tex".to_string(),
    outputformat: "html".to_string(),
    inputconverter: Some("import".to_string()),
    complex: true,
  };
  assert!(backend.add(&new_tex_to_html_service).is_ok());
  let service_registered_result = Service::find_by_name(service_name, &backend.connection);
  assert!(service_registered_result.is_ok());
  let service_registered = service_registered_result.unwrap();

  assert!(
    backend
      .register_service(&service_registered, &corpus_path)
      .is_ok()
  );
}
