use crate::http_methods::{delete_api, get_api, patch_api, post_api, put_api, ApiOutput};
use actix_web::web::{delete, get, patch, post, put, resource, scope, ServiceConfig};

const TAG: &str = "Anything";

pub(crate) fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/anything")
            .service(
                resource("")
                    .route(delete().to(delete_api))
                    .route(get().to(get_api))
                    .route(patch().to(patch_api))
                    .route(post().to(post_api))
                    .route(put().to(put_api)),
            )
            .service(
                resource("/{_:.*}")
                    .route(delete().to(delete_api))
                    .route(get().to(get_api))
                    .route(patch().to(patch_api))
                    .route(post().to(post_api))
                    .route(put().to(put_api)),
            ),
    );
}

// - Note：以下代码为了生成 OpenAPI 文档，并不会被实际接口调用 -

#[utoipa::path(
    tag = TAG,
    delete,
    path = "/anything",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn anything_delete_api() {}

#[utoipa::path(
    tag = TAG,
    get,
    path = "/anything",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn anything_get_api() {}

#[utoipa::path(
    tag = TAG,
    patch,
    path = "/anything",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn anything_patch_api() {}

#[utoipa::path(
    tag = TAG,
    post,
    path = "/anything",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn anything_post_api() {}

#[utoipa::path(
    tag = TAG,
    put,
    path = "/anything",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn anything_put_api() {}

// All anything
#[utoipa::path(
    tag = TAG,
    delete,
    path = "/anything/*",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn all_anything_delete_api() {}

#[utoipa::path(
    tag = TAG,
    get,
    path = "/anything/*",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn all_anything_get_api() {}

#[utoipa::path(
    tag = TAG,
    patch,
    path = "/anything/*",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn all_anything_patch_api() {}

#[utoipa::path(
    tag = TAG,
    post,
    path = "/anything/*",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn all_anything_post_api() {}

#[utoipa::path(
    tag = TAG,
    put,
    path = "/anything/*",
    responses(
        (status = 200, body = ApiOutput)
    )
)]
#[allow(dead_code)]
pub(crate) fn all_anything_put_api() {}
