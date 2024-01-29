use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use apiplay::config::read_config;
use apiplay::modules::music;
use apiplay::modules::music::domain::Playlist;
use apiplay::state::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = read_config();
    let stack: Vec<Playlist> = vec![];
    let playlists = Arc::new(Mutex::new(stack));
    HttpServer::new( move || {
        App::new()
            .app_data(web::Data::new(State{
                playlist: playlists.clone()
             }))
            .service(web::scope("/api")
                .configure(music::infra::endpoints::config)
        ) // sirve para agregar un conjunto de rutas o servicios bajo el ámbito "/api". scope es un contenedor que permite agrupar rutas y servicios bajo una cierta ruta o prefijo común.
    }) 
    .bind((config.host, config.port))?
    .run()
    .await
}