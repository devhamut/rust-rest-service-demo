use super::swagger_controller::get_swagger_ui;
use crate::service::greetings_service::{retrieve_hello, retrieve_hello_world};
use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_hello,
        get_hello_world,
        save_greetings
    ),
    components(
        schemas(HelloResponseJson, GreetingRequestJson)
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
struct HelloResponseJson {
    message: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
struct GreetingRequestJson {
    greeting_message: String,
    greeting_type: String,

}
/// Endpoint raíz que retorna un saludo
#[utoipa::path(
    get,
    path = "/",
    tag = "Greetings",
    responses(
        (status = 200, description = "Saludo exitoso", body = HelloResponseJson)
    )
)]
async fn get_hello() -> Json<HelloResponseJson> {
    Json(HelloResponseJson {
        message: retrieve_hello(),
    })
}

/// Endpoint que retorna "Hola mundo"
#[utoipa::path(
    get,
    path = "/hello-world",
    tag = "Greetings",
    responses(
        (status = 200, description = "Saludo exitoso", body = HelloResponseJson)
    )
)]
async fn get_hello_world() -> Json<HelloResponseJson> {
    Json(HelloResponseJson {
        message: retrieve_hello_world(),
    })
}

#[utoipa::path(
    post,
    path = "/save-greetings",
    tag = "Greetings",
    request_body = GreetingRequestJson,
    responses(
        (status = 200, description = "Saludo guardado exitosamente")
    )
)]
async fn save_greetings(Json(payload): Json<GreetingRequestJson>) -> Json<HelloResponseJson> {
    // Aquí podrías implementar la lógica para guardar el saludo en una base de datos o en memoria
    // Por ahora, simplemente retornamos el mismo mensaje que recibimos
    Json(HelloResponseJson {
        message: format!("Saludo guardado: {}", payload.greeting_message),
    })
}


/// Función que retorna el router con las rutas definidas
pub fn get_greetings_router() -> Router {
    let swagger_ui = get_swagger_ui::<ApiDoc>();

    Router::new()
        .route("/", get(get_hello))
        .route("/hello-world", get(get_hello_world))
        .route("/save-greetings", axum::routing::post(save_greetings))
        .merge(swagger_ui)
}