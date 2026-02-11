use super::swagger_controller::get_swagger_ui;
use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;

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

/// Función que retorna el router con las rutas definidas
pub fn get_greetings_router() -> Router {
    let swagger_ui = get_swagger_ui::<ApiDoc>();

    Router::new()
        .route("/", get(hello_root))
        .route("/hello-world", get(hello_world))
        .merge(swagger_ui)
}
