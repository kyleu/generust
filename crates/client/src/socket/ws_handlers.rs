use crate::ctx::ClientContext;

use anyhow::Result;
use js_sys::{ArrayBuffer, Uint8Array};
use std::rc::Rc;
use std::sync::RwLock;
use wasm_bindgen::prelude::{Closure, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{Blob, ErrorEvent, FileReader, MessageEvent};
use {{crate_name}}_core::ResponseMessage;

pub(crate) fn on_open(ctx: &Rc<RwLock<ClientContext>>) -> Result<()> {
  ctx.read().unwrap().on_open()
}

pub(crate) fn on_message(ctx: &Rc<RwLock<ClientContext>>, evt: MessageEvent) -> Result<()> {
  let data = evt.data();
  if data.is_instance_of::<ArrayBuffer>() {
    on_array_message(ctx, data)
  } else if data.is_instance_of::<Blob>() {
    on_blob_message(ctx, data)
  } else {
    on_text_message(ctx, data)
  }
}

fn on_array_message(ctx: &Rc<RwLock<ClientContext>>, data: JsValue) -> Result<()> {
  let buf: &ArrayBuffer = data.unchecked_ref();
  handle(ctx, parse_array_buffer(buf)?)
}

fn on_blob_message(ctx: &Rc<RwLock<ClientContext>>, data: JsValue) -> Result<()> {
  let fr = Rc::new(Box::new(FileReader::new().expect("!!!")));
  let cb: Box<dyn Fn()> = {
    let cb_fr = Rc::clone(&fr);
    let cb_ctx = Rc::clone(&ctx);
    Box::new(move || {
      let v: ArrayBuffer = cb_fr.result().unwrap().dyn_into::<ArrayBuffer>().unwrap();
      let rm = parse_array_buffer(&v).unwrap();
      match handle(&cb_ctx, rm) {
        Ok(_) => {}
        Err(e) => error!("Error processing response message: {:?}", e)
      };
    })
  };
  let onsuccess_callback = Closure::wrap(cb);
  fr.set_onload(Some(onsuccess_callback.as_ref().unchecked_ref()));
  onsuccess_callback.forget();
  fr.read_as_array_buffer(data.unchecked_ref()).expect("!!!");
  Ok(())
}

fn on_text_message(ctx: &Rc<RwLock<ClientContext>>, data: JsValue) -> Result<()> {
  match data.as_string() {
    Some(s) => handle(ctx, ResponseMessage::from_json(&s)?),
    None => Err(anyhow::anyhow!(format!("Can't convert received data to a string: {:?}", data)))
  }
}

fn handle(ctx: &Rc<RwLock<ClientContext>>, response: ResponseMessage) -> Result<()> {
  crate::message_handler::MessageHandler::handle(&ctx, response)
}

pub(crate) fn on_error(ctx: &Rc<RwLock<ClientContext>>, _e: ErrorEvent) -> Result<()> {
  ctx.read().unwrap().on_error()
}

pub(crate) fn on_close(ctx: &Rc<RwLock<ClientContext>>) -> Result<()> {
  ctx.read().unwrap().on_close()
}

fn parse_array_buffer(a: &ArrayBuffer) -> Result<ResponseMessage> {
  let buffy = Uint8Array::new(a);
  let mut v = vec![0; buffy.length() as usize];
  buffy.copy_to(&mut v);
  ResponseMessage::from_binary(&v)
}
