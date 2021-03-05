use std::collections::HashMap;
use std::time::SystemTime;

use duktape_rs::DukContext;

#[derive(Debug)]
pub struct Js {
    _ctx: DukContext
}

impl Default for Js {
    fn default() -> Self {
        Js::new()
    }
}


impl Drop for Js {
    fn drop(&mut self) {
        self._ctx.destroy()
    }
}

impl Js {
    pub fn new() -> Self {
        Js {
            _ctx: DukContext::new()
        }
    }

    pub fn addScript(&mut self, script: &str) {
        &self._ctx.eval_string(script).unwrap();
    }

    pub fn execute(&mut self, script: &str, func: &str, params: HashMap<&str, &str>) -> String {
        &self._ctx.eval_string(script).unwrap();

        for (key, value) in params.into_iter() {
            let v_set = format!("var {} = \"{}\"", key, value);
            &self._ctx.eval_string(&v_set).unwrap();
        }

        let result = &self._ctx.eval_string(func).unwrap();

        return result.as_str().unwrap();
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
    fn should_add_script() {
        let mut js = Js::new();
        let mut map = HashMap::new();
        map.insert("b", "hello");
        map.insert("c", " world");

        js.addScript("function a(b,c){return b+c;}");
        let result = js.execute("function d(b,c){return b;}", "a(b,c)", map);

        assert_that!(result.as_str(), is(equal_to("hello world")));
    }
}
