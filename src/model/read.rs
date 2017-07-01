extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct Type {
    id: u32,
    title: String,
    sort: u8
}

#[derive(Serialize, Deserialize)]
pub struct Types {
    pub types: Vec<Type>
}


pub fn get_types() -> Option<Types> {
    let t1 = Type {
        id: 1,
        title: "我才去".into(),
        sort: 1
    };

    let mut types = Types{ types: vec![] };
    types.types.push(t1);

    Some(types)

    // None
}
