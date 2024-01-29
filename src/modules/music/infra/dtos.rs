use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub id:usize
}

#[derive(Deserialize)]
pub struct CreatePlaylist{
    pub name: String
}

#[derive(Deserialize)]
pub struct PartialUpdatePlaylist{
    pub name : Option<String>,
}