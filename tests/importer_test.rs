extern crate cortex;
use cortex::importer::*;
use cortex::backend::Backend;
use std::vec::Vec;
use std::fs;
use std::io::{Error};

fn assert_files(files:Vec<&str>) -> Result<(),std::io::Error> {
  for file in files.iter() {
    let meta = fs::metadata(file.clone());
    assert!(meta.is_ok());
    assert!(meta.unwrap().is_file());
    // They're also temporary, so delete them
    try!(fs::remove_file(file.clone())); 
  }
  Ok(()) }

fn assert_dirs(dirs : Vec<&str>) -> Result<(),std::io::Error> {
  for dir in dirs.iter() {
    let meta = fs::metadata(dir.clone());
    assert!(meta.is_ok());
    assert!(meta.unwrap().is_dir());
    // They're also temporary, so delete them
    try!(fs::remove_dir(dir.clone())); 
  }
  Ok(()) }

#[test]
fn can_import_simple() {
  let default_backend = Backend::default();
  let importer = Importer {
    corpus: default_backend.add_corpus("tests/data/".to_string(), false),
    backend: default_backend };
  
  println!("-- Testing simple import");
  assert_eq!( importer.process(), Ok(()) );
}

#[test]
fn can_import_complex() {
  let default_backend = Backend::default();
  let importer = Importer {
    corpus: default_backend.add_corpus("tests/data/".to_string(), true),
    backend: Backend::default() };

  
  println!("-- Testing complex import");
  assert_eq!( importer.process(), Ok(()) );

  let repeat_importer = Importer {
    corpus: default_backend.add_corpus("tests/data/".to_string(), false),
    backend: Backend::default() };

  
  println!("-- Testing repeated complex import (successful and no-op)");
  assert_eq!( repeat_importer.process(), Ok(()) );

  let files_removed_ok = assert_files(vec![
    "tests/data/9107/hep-lat9107001/hep-lat9107001.zip",
    "tests/data/9107/hep-lat9107002/hep-lat9107002.zip",
    ]);
  assert!(files_removed_ok.is_ok());
  let dirs_removed_ok = assert_dirs(vec![
    "tests/data/9107/hep-lat9107001",
    "tests/data/9107/hep-lat9107002",
    "tests/data/9107"
  ]);
  assert!(dirs_removed_ok.is_ok());
  
}