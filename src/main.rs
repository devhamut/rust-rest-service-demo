mod controller;
mod service;
mod persistance;

use controller::greetings_controller::get_greetings_router;
use std::env;

#[tokio::main]
async fn main() {
    // Cargamos las variables de entorno desde el archivo .env
    dotenvy::dotenv().ok();

    // Obtenemos las variables del .env o usamos valores por defecto
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_address = format!("{}:{}", host, port);

    // Lanzamos el servidor (equivalente a Tomcat/Netty)
    println!("🚀 Servidor iniciado en http://{}", bind_address);
    println!(
        "📚 Documentación Swagger disponible en http://{}/swagger-ui/",
        bind_address
    );

    // Definimos la ruta
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();

    // Obtenemos el router con las rutas definidas
    let app = get_greetings_router();

    // Iniciamos el servidor con Axum
    axum::serve(listener, app).await.unwrap();
}
