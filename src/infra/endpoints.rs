use actix_web::{get, web, Responder, HttpResponse}; 

#[get("/version")]
async fn version() -> impl Responder{ //Funcion asincrona llamada version que va a regresar algo que implementa un responder
    HttpResponse::Ok()
        .body("1.50.0")
} 

pub fn config(cfg: &mut web::ServiceConfig){ // función publica llamada config, con la variable cfg con modificador &mut que significa que es la referencia es mutable (puede modificar el objeto referenciado) y el tipo de objeto al que cfg hace referencia. "ServiceConfig" es un tipo proporcionado por la biblioteca que se utiliza para configurar servicios y rutas en la app web.
    cfg.service(version); //service es un método de ServiceConfig que se utiliza para agregar un servicio al conjunto de servicios que serán manejados por la app web.
}