use wasm_bindgen::prelude::*;
use factorial::Factorial;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("should have a body on document");
    let val = document.create_element("p")?;

    let string_val = "Hello from rust!";

    val.set_text_content(Some(string_val));

    body.append_child(&val)?;

    Ok(())
}

