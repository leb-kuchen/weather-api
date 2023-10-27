use schemafy_lib::Expander;
use serde::{Deserialize, Serialize};
pub mod swagger;
pub fn gen_schema() {
    let json = std::fs::read_to_string("swagger.json").expect("Read schema JSON file");

    let schema = serde_json::from_str(&json).unwrap();
    let mut expander = Expander::new(Some("Schema"), "::schemafy_core::", &schema);

    let code = expander.expand(&schema);
    println!("{code}")
}
