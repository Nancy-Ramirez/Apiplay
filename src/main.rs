use actix_web::{web, App, HttpServer};
use apiplay::infra;
use apiplay::config::read_config;
use apiplay::modules::music;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = read_config();
    HttpServer::new( || {
        App::new()
            .service(web::scope("/api")
                .configure(infra::endpoints::config)
                .configure(music::infra::endpoints::config)
        ) // sirve para agregar un conjunto de rutas o servicios bajo el ámbito "/api". scope es un contenedor que permite agrupar rutas y servicios bajo una cierta ruta o prefijo común.
    }) 
    .bind((config.host, config.port))?
    .run()
    .await
}