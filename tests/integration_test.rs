extern crate core;
#[macro_use]
extern crate hamcrest;
extern crate tempdir;

use std::io;

use hamcrest::prelude::*;
use log::{debug, info};
use tempdir::TempDir;

#[test]
fn should_load_plugin() {
    let temp_dir = TempDir::new("data").unwrap();

    let ctx = core::init_context(temp_dir.path().to_str().unwrap());
    let mut engine = core::init_engine(ctx);

    let plugins = engine.list();

    assert_that!(plugins.len(), is(equal_to(0)));

    let file_content = std::fs::read_to_string("tests/files/1.xml").expect("read file fail");

    engine.load(&file_content);

    let plugins = engine.list();

    assert_that!(plugins.len(), is(equal_to(1)));

    let plugin = plugins.get(0).unwrap();

    info!("plugin:{:?}", plugin);
}

#[test]
fn should_get_hots() {
    let temp_dir = TempDir::new("data").unwrap();

    let ctx = core::init_context(temp_dir.path().to_str().unwrap());
    let mut engine = core::init_engine(ctx);

    let plugins = engine.list();

    let file_content = std::fs::read_to_string("tests/files/1.xml").expect("read file fail");

    engine.load(&file_content);

    let uid = "bc9e3e3b-c6e8-4d84-b976-81cce04e4a61";

    let result = engine.find(uid);

    assert_that!(result.is_ok(), is(true));

    let p = result.ok().unwrap();

    let books = p.getHots();

    assert_that!(books.len(), is(greater_than(1)));
}
