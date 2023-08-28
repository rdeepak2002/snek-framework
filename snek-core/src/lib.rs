use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn python_to_html() -> String {
    let website_code = r#"
        <div>
            <h1>Header text</h1>
            <p>Paragraph text</p>
        </div>
    "#;
    String::from(website_code)
}
