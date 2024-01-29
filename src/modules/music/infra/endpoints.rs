use actix_web::{patch, put, delete, get, post, web, Responder, error};
use crate::state::State;
use super::super::domain::Playlist; 
use super::dtos::{Info, CreatePlaylist, PartialUpdatePlaylist};

//*Función GET All*/
#[get("/playlist")] 
async fn playlist(data: web::Data<State>) -> impl Responder {

    let playlists = data.playlist.lock().expect("bad state");
    web::Json(playlists.clone())
}
//*Función get_id */
#[get("/playlist/{id}")] 
async fn get_playlist(info: web::Path<Info>, data: web::Data<State>) -> impl Responder{
    
    let playlists = data.playlist.lock().expect("bad state");
    let p1=playlists[info.id].clone();
    web::Json(p1)
}

//*Funcion POST */
#[post("/playlist")]
async fn create_playlist(dto: web::Json<CreatePlaylist>, data: web::Data<State>) -> impl Responder {
    let mut playlists = data.playlist.lock().expect("bad state");

    let p1= Playlist{
        name: dto.name.clone(),
        songs: vec![],
    };
    playlists.push(p1.clone());

    web::Json(p1)
}

//*Función DELETE */
#[delete("/playlist/{id}")]
async fn delete_playlist(info: web::Path<Info>, data: web::Data<State>) -> impl Responder{
    let mut playlists = data.playlist.lock().expect("bad state");
    let p1 = playlists.remove(info.id).clone();
    web::Json(p1)
}

//*Función Update All */
#[put("/playlist/{id}")]
async fn update_playlist(info: web::Path<Info>, dto: web::Json<CreatePlaylist>, data: web::Data<State>) -> impl Responder {
    let mut playlists = data.playlist.lock().expect("bad state");

    if let Some(play)= playlists.get_mut(info.id){
        play.name = dto.name.clone();
        Ok(web::Json(play.clone()))
        } else {
            Err(error::ErrorNotFound("Playlist not found"))
        }

    }

//*Función Patch  */
#[patch("/playlist/{id}")]
async fn partial_update_playlist(info: web::Path<Info>, dto: web::Json<PartialUpdatePlaylist>, data: web::Data<State> ) -> impl Responder{
    let mut playlists = data.playlist.lock().expect("bad state");
    
    if let Some(play) = playlists.get_mut(info.id){
        if let Some(new_name) = &dto.name {
            play.name = new_name.clone();
        }
        Ok(web::Json(play.clone()))
    } else {
        Err(error::ErrorNotFound("Playlist not found"))
    }
}

pub fn config(cfg: &mut web::ServiceConfig){  // función publica llamada config, con la variable cfg con modificador &mut que significa que es la referencia es mutable (puede modificar el objeto referenciado) y el tipo de objeto al que cfg hace referencia. "ServiceConfig" es un tipo proporcionado por la biblioteca que se utiliza para configurar servicios y rutas en la app web.

    cfg.service(playlist); //service es un método de ServiceConfig que se utiliza para agregar un servicio al conjunto de servicios que serán manejados por la app web.
    cfg.service(get_playlist);
    cfg.service(create_playlist);
    cfg.service(delete_playlist);
    cfg.service(update_playlist);
    cfg.service(partial_update_playlist);
}
