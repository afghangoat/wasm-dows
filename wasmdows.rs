use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, Document, Window, HtmlElement};

/*#[wasm_bindgen]
fn greet(mut name: &str) -> String {
	name="";
	println!("{}",name);
	format!("Hello!")
}*/
#[wasm_bindgen]
pub fn main(){
}
#[wasm_bindgen]
pub fn change_image_src(id: &str, new_src: &str) {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let img = document.get_element_by_id(id).expect("should have an element with the given ID");
    let img: HtmlImageElement = img.dyn_into::<HtmlImageElement>().expect("element should be an HtmlImageElement");
    img.set_src(new_src);
}

#[wasm_bindgen]
pub fn set_background(awidth: f32,aheight: f32){
	let ratio:f32=awidth/aheight;
	if ratio>1.7 {
		change_image_src("bg-img","./img/back_wide.jpg");
	} else {
		change_image_src("bg-img","./img/back_normal.jpg");
	}
}
#[wasm_bindgen]
pub fn fetch_txt_data(aid: &str) -> String{
	let mut text_value="";
	match aid{
		"readme"=>text_value="Hi there!\nI made this desktop imitation to practice rust! Feel free to look around, you may find some secrets.",
		"recipe1"=>text_value="Chips:\n-Slice it up, paint it with a little bit of oil.\n-place them on heatpapers on a plate.\n-heat it in the microwave on 750Watts for 3 minutes.\n-flip them and continue the microwaving for 2 minutes.\n-Salt them.",
		_=>text_value="Failed to load file..."
	}
	return text_value.to_string();
}
//main.exe
//rustc src/main.rs
//import init, { greet } from './pkg/your_wasm_package.js';
//wasm-pack build --target web