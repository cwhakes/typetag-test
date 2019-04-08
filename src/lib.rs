use wasm_bindgen::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data ( Box<dyn MyTrait> );

#[derive(Debug, Serialize, Deserialize)]
struct MyType(String);

#[typetag::serde(tag = "type")]
trait MyTrait: std::fmt::Debug {}

#[typetag::serde]
impl MyTrait for MyType {}

impl Data {
    fn new(my_trait: Box<dyn MyTrait>) -> Data {
        Data(my_trait)
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    let data = Data::new(Box::new(MyType("foo".to_string())) as Box<MyTrait>);
    alert(&format!("{:?}", &data));
    let text = serde_json::to_string(&data).unwrap();
    alert(&format!("{}", text));
    let new_data = serde_json::from_str::<Data>(&text);
    alert(&format!("{:?}", new_data));

}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}