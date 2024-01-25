use actix_web::{get, web, Responder};
use serde::Deserialize;
use super::super::domain::Playlist; //super se refiere al módulo padre, al utilizarlo dos veces se van dos niveles arriba.

#[derive(Deserialize)]
struct Info {
    id:usize
}
//* */
#[get("/playlist")] // indica que la funcion decorada con esto manejará solicitudes HTTP GET en la ruta "/playlist"
async fn playlist() -> impl Responder {
    let mut playlists: Vec<Playlist> = vec![]; //variable mutante del tipo vector de Playlists que estrá vacío.
    let p1: Playlist = Playlist{
        name: "Lisseth 2022".to_string(),
        songs: vec![]
    };
    playlists.push(p1);//guardamos la variable que contiene información del tipo Playlist en el vector mutante definido anteriormente.
    
    let p2: Playlist = Playlist{
        name: "Nancy 2023".to_string(),
        songs: vec![]
    }; //Creamos una variable tipo Playlist para llenar un dato de información

    playlists.push(p2); 

    //Nos permite convertir estructuras que ya tenemos dentro de rust a una estructura serializada o forma de texto
    web::Json(playlists)
}

#[get("/playlist/{id}")] // indica que la funcion decorada con esto manejará solicitudes HTTP GET en la ruta "/playlist/{id}" donde se le solicita un id para ver los datos de un objeto en especifico.
async fn get_playlist(info: web::Path<Info>) -> impl Responder{
    let playlists: Vec<Playlist> = vec![Playlist{
        name: "Lisseth 2022".to_string(),
        songs: vec![]
    }, Playlist
    {
        name: "Nancy 2023".to_string(),
        songs: vec![]
    } ]; // Guardamos de una vez la información en el vector

    let p1 = playlists[info.id].clone(); //Le asignamos el valor a mostrar a la variable segun el id ingresado, le asignamos clone() para obtener los datos
    web::Json(p1) //imprime
}

pub fn config(cfg: &mut web::ServiceConfig){  // función publica llamada config, con la variable cfg con modificador &mut que significa que es la referencia es mutable (puede modificar el objeto referenciado) y el tipo de objeto al que cfg hace referencia. "ServiceConfig" es un tipo proporcionado por la biblioteca que se utiliza para configurar servicios y rutas en la app web.

    cfg.service(playlist); //service es un método de ServiceConfig que se utiliza para agregar un servicio al conjunto de servicios que serán manejados por la app web.
    cfg.service(get_playlist);
}