use core::fmt;
use std::fmt::Formatter;

use log::{debug, info};

use crate::engine::{Book, DataConfig, ScriptCode};
use crate::js::js::Js;

#[derive(Default, Debug)]
pub struct Plugin {
    pub js: Js,
    pub ver: i32,
    pub guid: String,
    pub title: String,
    pub code: String,
    pub url: String,
    pub hots: DataConfig,
    pub codes: Vec<ScriptCode>,
}

impl Plugin {
    pub fn getHots(&self) -> Vec<Book> {
        let hots_config = &self.hots;

        let mut response_content = String::new();
        match hots_config.method.to_lowercase().as_str() {
            "post" => {}
            _ => {
                //default is get method
                response_content = reqwest::blocking::get(&hots_config.url).unwrap().text().unwrap();
            }
        }

        debug!("get response from {} as {}", hots_config.url, response_content);

        vec![Default::default()]
    }
}


impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Plugin (id:{}, version:{}, name:{})",
            self.guid, self.ver, self.title
        )
    }
}
