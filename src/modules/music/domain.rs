use serde::Serialize;
//dominio propio de la aplicación como entidades para definir o objetos con sus propiedades.
//Se declaran pub para hacerlos accesibles desde fuera del módulo donde están definidos.

#[derive(Debug, Serialize, Clone)]
pub struct Song{
    pub name: String,
    pub author: String,
    pub duration_ms: u16
}
#[derive(Debug, Serialize, Clone)]
pub struct Playlist{
    pub name : String,
    pub songs: Vec<Song>
}