pub mod read;

#[derive(Serialize, Deserialize)]
pub struct Type {
    pub id: u32,
    pub title: String,
    pub sort: u8
}

#[derive(Serialize, Deserialize)]
pub struct Types {
    pub types: Vec<Type>
}

