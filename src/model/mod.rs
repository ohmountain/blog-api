pub mod read;
pub mod write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
    pub id: Option<u32>,
    pub title: String,
    pub sort: u8
}

#[derive(Serialize, Deserialize)]
pub struct Types {
    pub types: Vec<Type>
}


#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: Option<u32>,
    pub type_id: u32,
    pub title: String,
    pub created_at: String,
    pub updated_at: String
}

