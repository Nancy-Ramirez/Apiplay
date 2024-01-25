use std::env;
use dotenv::dotenv;

pub struct Config {
    pub port: u16,
    pub host: String
}

pub fn read_config() -> Config{ //funcion para leer las configuraciones y nos debe revolver una instancia de la estructura Config
    dotenv().ok(); // para que lea y busque el archivo .env, lo cargue en los valores de entorno y podamos tener acceso a ellas. 

    Config{
        port: env::var("PORT").expect("No port defined").parse().expect("Bad port definition"),
        host: env::var("HOST").expect("No host defined"),
    }
}