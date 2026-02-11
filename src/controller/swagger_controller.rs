use utoipa_swagger_ui::SwaggerUi;

pub fn get_swagger_ui<T: utoipa::OpenApi>() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", T::openapi())
}
