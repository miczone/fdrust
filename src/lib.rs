use wasm_bindgen::prelude::*;

#[cfg(feature = "dosomething")]
pub use self::with_cfg::dosomething;

#[cfg(not(feature = "dosomething"))]
pub use self::without_cfg::dosomething;

#[cfg(feature = "dosomething")]
mod with_cfg {
    pub fn dosomething() {
        println!("config option");
    }
}

#[cfg(not(feature = "dosomething"))]
mod without_cfg {
    pub fn dosomething() {
        println!("no config option");
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}