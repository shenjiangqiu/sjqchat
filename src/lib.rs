pub mod client;
use client::app::SendButton;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
#[wasm_bindgen]
pub fn run()->Result<(),JsValue> {
    yew::start_app::<SendButton>();
    Ok(())
}
