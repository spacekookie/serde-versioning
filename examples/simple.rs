#![feature(proc_macro)]

extern crate serde;
extern crate serde_versions;
use serde_versions::version;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[version]
struct Model {
    name: String,
    #[version]
    age: u32,
}

// #[derive(Serialize, Deserialize)]
// #[serde(tag = "schema_version")]
// enum MyStruct {
//     #[serde(rename = "1")]
//     V1 { name: String },
//     #[serde(rename = "2")]
//     V2 { name: String, age: u8 },
// }

fn main() {
    // let v1 = MyStruct::V1 {
    //     name: "Kate".into(),
    // };

    // let v2 = MyStruct::V2 {
    //     name: "Kate".into(),
    //     age: 25,
    // };

    // println!("V1 ==> {}", serde_json::to_string_pretty(&v1).unwrap());
    // println!("V2 ==> {}", serde_json::to_string_pretty(&v2).unwrap());

    // // This is a V2 model
    // let v2 = Model {
    //     name: "Kate".into(),
    //     age: 25,
    // };

    // // This could either be v1 or v3, If not otherwise
    // //   specified, it will default to the newer version
    // let v3 = Model {
    //     name: "Kate".into(),
    // };
}
