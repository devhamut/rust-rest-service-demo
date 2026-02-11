use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        hello_root,
        hello_world
    ),
    components(
        schemas(HelloResponse)
    ),
    tags(
        (name = "Greetings", description = "Endpoints de saludos")
    ),
    info(
        title = "REST Service API",
        version = "1.0.0",
        description = "Una API REST simple en Rust con Axum"
    )
)]
struct ApiDoc;

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
struct HelloResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    // Definimos la ruta
    let app = get_router();

    // Lanzamos el servidor (equivalente a Tomcat/Netty)
    println!("🚀 Servidor iniciado en http://0.0.0.0:3000");
    println!("📚 Documentación Swagger disponible en http://0.0.0.0:3000/swagger-ui/");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_router() -> Router {
    Router::new()
        .route("/", get(hello_root))
        .route("/hello-world", get(hello_world))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}

/// Endpoint raíz que retorna un saludo
#[utoipa::path(
    get,
    path = "/",
    tag = "Greetings",
    responses(
        (status = 200, description = "Saludo exitoso", body = HelloResponse)
    )
)]
async fn hello_root() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "¡Hola desde Rust!".to_string(),
    })
}

/// Endpoint que retorna "Hola mundo"
#[utoipa::path(
    get,
    path = "/hello-world",
    tag = "Greetings",
    responses(
        (status = 200, description = "Saludo exitoso", body = HelloResponse)
    )
)]
async fn hello_world() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "¡Hola, mundo!".to_string(),
    })
}
