use super::anything;
use super::auth;
use super::dynamic_data;
use super::http_methods;
use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::HttpBuilder;
use utoipa::openapi::security::SecurityScheme;
use utoipa::Modify;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "httpbin-rs", description = "httpbin implemented in Rust"),
    paths(
        //  Http Methods
        http_methods::delete_api,
        http_methods::get_api,
        http_methods::patch_api,
        http_methods::post_api,
        http_methods::put_api,

        // Auth
        auth::basic_auth_api,
        auth::bearer_auth_api,

        // Anything
        anything::anything_delete_api,
        anything::anything_get_api,
        anything::anything_patch_api,
        anything::anything_post_api,
        anything::anything_put_api,
        anything::all_anything_delete_api,
        anything::all_anything_get_api,
        anything::all_anything_patch_api,
        anything::all_anything_post_api,
        anything::all_anything_put_api,

        // Dynamic data
        dynamic_data::delete_delay_api,
        dynamic_data::get_delay_api,
        dynamic_data::patch_delay_api,
        dynamic_data::post_delay_api,
        dynamic_data::put_delay_api
    ),
    modifiers(&SecurityAddon)
)]
pub(crate) struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

        components.add_security_scheme(
            "basicAuth",
            SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
        );
    }
}
