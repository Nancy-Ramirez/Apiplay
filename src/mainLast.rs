use actix_web::{get, web, App, HttpServer, Responder}; //Importamos de la biblioteca de actix 

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder{
    format!("Hello {name}!")
}

#[actix_web::main] //Usamos un macro para utilizar funciones asincronas.
async fn main() -> std::io::Result<()> { //Función principal
    HttpServer::new(||{ //al metodo new le asignamos una función
        App::new() //creamos una aplicación de actix-web
            .route("/hello", web::get().to(|| async { "Hello World" })) //creamos una ruta y le estamos pasando el endpoint "/hello", se configura la ruta para utilizar el metodo "GET", con un cierre sin parametros (||) definiendo un bloque asincronico que produce la cadena "Hello World"
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))? //bind es un metodo que especifica la dirección IP y el puerto, la interrogancion maneja cualquier error que pueda ocurrir durante el proceso. Enlaza el servidor de actix a una dirección IP y puertos especificos.
    .run() //Inicia la ejecución  del servidor web
    .await // Espera la finalización de una operación asincrona
}
