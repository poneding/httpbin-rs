use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "httpbin-rs", description = "httpbin implemented in Rust"),
    paths()
)]
pub(crate) struct ApiDoc;
