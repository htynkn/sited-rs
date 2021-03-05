use std::collections::HashMap;
use std::time::SystemTime;

use rusty_v8 as v8;

#[derive(Debug)]
pub struct Js {}

impl Default for Js {
    fn default() -> Self {
        Js::new()
    }
}

impl Js {
    pub fn new() -> Self {
        Js {}
    }

    pub fn addScript(&mut self, script: &str) {}

    pub fn execute(&mut self, script: &str, func: &str, params: HashMap<&str, &str>) -> String {
        "".to_owned()
    }

    pub fn executeCommand(&mut self, command: &str) -> String {
        let platform = v8::new_default_platform().unwrap();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);

        // Create a new context.
        let context = v8::Context::new(handle_scope);

        // Enter the context for compiling and running the hello world script.
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // Create a string containing the JavaScript source code.
        let code = v8::String::new(scope, command).unwrap();

        // Compile the source code.
        let script = v8::Script::compile(scope, code, None).unwrap();
        // Run the script to get the result.
        let result = script.run(scope).unwrap();

        // Convert the result to a string and print it.
        let result = result.to_string(scope).unwrap();

        return result.to_rust_string_lossy(scope);
    }
}

#[cfg(test)]
mod tests {
    use hamcrest::{equal_to, is};
    use hamcrest::prelude::*;

    use super::*;

    #[test]
    fn should_exec_basic_func() {
        let mut js = Js::new();
        let mut map = HashMap::new();
        map.insert("b", "hello");
        map.insert("c", " world");
        let result = js.execute("function a(b,c){return b+c;}", "a(b,c)", map);

        assert_that!(result.as_str(), is(equal_to("hello world")));
    }

    #[test]
    fn should_run_command() {
        let mut js = Js::new();

        let result = js.executeCommand("1+1");

        assert_that!(result.as_str(),is(equal_to("2")));
    }

    #[test]
    fn should_add_script() {
        let mut js = Js::new();
        let mut map = HashMap::new();
        map.insert("b", "hello");
        map.insert("c", " world");

        js.addScript("function a(b,c){return b+c;}");

        let result = js.execute("function d(b,c){return b;}", "a(b,c)", map);

        assert_that!(result.as_str(), is(equal_to("hello world")));
    }

    #[test]
    fn should_enable_load_cheerio() {
        let mut js = Js::new();

        let file_content = std::fs::read_to_string("tests/files/cheerio.js").expect("read file fail");

        js.addScript(&file_content);
    }
}
