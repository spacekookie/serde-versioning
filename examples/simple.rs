extern crate serde;

#[macro_use]
extern crate serde_versions;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Versioning)]
#[version(version = "1")]
struct Model {
    name: String,
    #[version(added = "2")]
    age: u32,
}

fn main() {
    let m = Model {
        name: "Kate".into(),
        age: 25,
    };
}
