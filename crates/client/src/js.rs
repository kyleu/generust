use wasm_bindgen::prelude::wasm_bindgen;

// Imports
#[wasm_bindgen]
extern "C" {
  #[allow(unsafe_code)]
  #[wasm_bindgen(js_namespace = console)]
  pub(crate) fn log(s: &str, style: &str);

  #[allow(unsafe_code)]
  #[wasm_bindgen(js_namespace = rustimate)]
  pub(crate) fn notify(level: &str, content: &str);
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct JsClient {
  ctx: std::rc::Rc<std::sync::RwLock<crate::ctx::ClientContext>>
}

impl Default for JsClient {
  fn default() -> Self {
    Self::new()
  }
}

#[wasm_bindgen]
impl JsClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let ctx = match crate::ctx::ClientContext::new() {
      Ok(c) => c,
      Err(e) => panic!("Cannot construct ClientContext: {}", e)
    };

    JsClient { ctx }
  }

  #[allow(unused_qualifications)]
  pub fn on_event(&self, t: &str, k: &str, v: &str) {
    let mut ctx = self.ctx.write().unwrap();
    match ctx.on_event(t, k, v) {
      Ok(_) => {}
      Err(e) => error!("Error encountered running [on_event]: {}", e)
    };
  }
}
