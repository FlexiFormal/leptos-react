use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct JsDisplay(pub JsValue);
impl std::fmt::Display for JsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = self.0.as_string() {
            return f.write_str(&v);
        }
        if let Some(v) = self.0.as_f64() {
            return write!(f, "num {v}");
        }
        if let Some(v) = self.0.as_bool() {
            return write!(f, "boolean {v}");
        }
        if let Ok(js) = web_sys::js_sys::JSON::stringify(&self.0) {
            let s: String = js.into();
            return f.write_str(&s);
        }
        write!(f, "object {:?}", self.0)
    }
}
