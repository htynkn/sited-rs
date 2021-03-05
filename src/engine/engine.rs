use std::borrow::BorrowMut;
use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};

use log::{debug, info};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

use crate::engine::{Context, DataConfig, ScriptCode};
use crate::engine::plugins::Plugin;
use crate::errors;
use crate::errors::EngineError;

use super::xml::attribute::OwnedAttribute;
use super::xml::name::OwnedName;
use super::xml::reader::EventReader;
use super::xml::reader::XmlEvent;

#[derive(Debug)]
pub struct Engine {
    pub context: Context,
    _pm: PluginManager,
}

#[derive(Default, Debug)]
struct PluginManager {
    _plugins: Vec<Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        let mut pm: PluginManager = Default::default();
        pm._plugins = vec![];
        pm
    }

    pub fn add(&mut self, p: Plugin) {
        self._plugins.push(p);
    }

    pub fn all(&self) -> &Vec<Plugin> {
        &self._plugins
    }
}

impl Engine {
    pub fn new(context: Context) -> Engine {
        Engine::initLogger(&context);
        Engine {
            context,
            _pm: PluginManager::new(),
        }
    }

    fn initLogger(context: &Context) {
        let stdout = ConsoleAppender::builder().build();
        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
            .build(Path::new(&context.data_path).join("info.log"))
            .unwrap();
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("file", Box::new(file)))
            .build(Root::builder()
                .appender("stdout").appender("file")
                .build(LevelFilter::Debug))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }

    pub fn list(&self) -> &Vec<Plugin> {
        &self._pm._plugins
    }

    pub fn find(&self, uid: &str) -> Result<&Plugin, EngineError> {
        for p in self._pm.all() {
            let p: &Plugin = p;
            if p.guid.eq(uid) {
                return Result::Ok(p);
            }
        }
        return Result::Err(EngineError {});
    }

    pub fn load(&mut self, str: &str) {
        let reader = EventReader::from_str(str);

        let mut plugin: Plugin = Default::default();
        let mut tag: String = String::new();
        let mut pre_tag: String = String::new();
        for e in reader {
            match e {
                Ok(XmlEvent::StartElement {
                       name, attributes, ..
                   }) => match name {
                    OwnedName { local_name, .. } => {
                        pre_tag = tag;
                        tag = local_name;
                        match tag.as_str() {
                            "sited" => parse_sited(&mut plugin, attributes),
                            "meta" => parse_meta(&mut plugin, attributes),
                            "hots" => parse_hots(&mut plugin, attributes),
                            _ => {
                                match format!("{}_{}", pre_tag, tag).as_str() {
                                    "require_item" => parse_require_item(&mut plugin, attributes),
                                    _ => {}
                                }
                            }
                        }
                    }
                },
                Ok(XmlEvent::CData(data)) => match tag.as_str() {
                    "code" => {
                        plugin.code = data;
                    }
                    _ => {}
                },
                Ok(XmlEvent::Characters(data)) => match tag.as_str() {
                    "title" => {
                        plugin.title = data;
                    }
                    _ => {}
                },
                Ok(_) => {}
                Err(_) => {}
            }
        }

        debug!("load plugin success for id: {}", plugin.guid);

        self._pm.add(plugin);
    }
}

fn parse_require_item(p: &mut Plugin, attributes: Vec<OwnedAttribute>) {
    let mut script_code: ScriptCode = Default::default();
    for a in attributes {
        match a.name {
            OwnedName { local_name, .. } => match local_name.as_str() {
                "url" => script_code.url = a.value,
                "id" => script_code.id = a.value,
                _ => {}
            },
        }
    }
    p.codes.push(script_code);
}

fn parse_hots(p: &mut Plugin, attributes: Vec<OwnedAttribute>) {
    let mut data_config: DataConfig = Default::default();
    for a in attributes {
        match a.name {
            OwnedName { local_name, .. } => match local_name.as_str() {
                "cache" => data_config.cache = a.value,
                "title" => data_config.title = a.value,
                "method" => data_config.method = a.value,
                "url" => data_config.url = a.value,
                "header" => data_config.header = a.value,
                "parse" => data_config.parse = a.value,
                _ => {}
            },
        }
    }
    p.hots = data_config;
}

fn parse_meta(p: &mut Plugin, attributes: Vec<OwnedAttribute>) {
    for a in attributes {
        match a.name {
            OwnedName { local_name, .. } => match local_name.as_str() {
                "guid" => p.guid = a.value,
                _ => {}
            },
        }
    }
}

fn parse_sited(p: &mut Plugin, attributes: Vec<OwnedAttribute>) {
    for a in attributes {
        match a.name {
            OwnedName { local_name, .. } => match local_name.as_str() {
                "ver" => p.ver = a.value.parse::<i32>().unwrap(),
                _ => {}
            },
        }
    }
}
