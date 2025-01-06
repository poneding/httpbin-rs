use super::anything;
use super::http_methods;
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
    )
)]
pub(crate) struct ApiDoc;
