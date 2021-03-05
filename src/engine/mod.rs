extern crate xml;

use core::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::{Path, PathBuf};

use self::xml::attribute::OwnedAttribute;
use self::xml::EventReader;
use self::xml::name::OwnedName;
use self::xml::reader::XmlEvent;

pub mod engine;
pub mod plugins;

#[derive(Default, Debug)]
pub struct ScriptCode {
    pub url: String,
    pub id: String,
}

#[derive(Default, Debug)]
pub struct DataConfig {
    pub cache: String,
    pub title: String,
    pub method: String,
    pub parse: String,
    pub url: String,
    pub header: String,
}

#[derive(Default, Debug)]
pub struct Book {
    pub name: String,
    pub logo: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Context {
    pub data_path: String,
}

impl Context {
    pub fn new(data_path: &str) -> Self {
        Context {
            data_path: data_path.to_string(),
        }
    }
}
