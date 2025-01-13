use crate::http_methods::{anything, ApiOutput};
use actix_web::{
    web::{delete, get, patch, post, put, resource, scope, Bytes, Json, Path, ServiceConfig},
    HttpRequest,
};
use std::time::Duration;
use tokio::time::sleep;

const TAG: &str = "Dynamic data";

pub(crate) fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/delay/{delay}").service(
            resource("")
                .route(delete().to(delete_delay_api))
                .route(get().to(get_delay_api))
                .route(post().to(post_delay_api))
                .route(put().to(put_delay_api))
                .route(patch().to(patch_delay_api)),
        ),
    );
}

#[utoipa::path(
    tag = TAG,
    delete,
    path = "/delay/{delay}",
    responses(
        (status=200, description="Returns a delayed response (max of 10 seconds).", body=ApiOutput)
    ),
    params(
        ("delay"=u64, Path, description="Delay in seconds.")
    )
)]
pub(crate) async fn delete_delay_api(
    req: HttpRequest,
    data: Bytes,
    path: Path<u64>,
) -> Json<ApiOutput> {
    delay_anything(req, data, path).await
}

#[utoipa::path(
    tag = TAG,
    get,
    path = "/delay/{delay}",
    responses(
        (status=200, description="Returns a delayed response (max of 10 seconds).", body=ApiOutput)
    ),
    params(
        ("delay"=u64, Path, description="Delay in seconds.")
    )
)]
pub(crate) async fn get_delay_api(
    req: HttpRequest,
    data: Bytes,
    path: Path<u64>,
) -> Json<ApiOutput> {
    delay_anything(req, data, path).await
}

#[utoipa::path(
    tag = TAG,
    post,
    path = "/delay/{delay}",
    responses(
        (status=200, description="Returns a delayed response (max of 10 seconds).", body=ApiOutput)
    ),
    params(
        ("delay"=u64, Path, description="Delay in seconds.")
    )
)]
pub(crate) async fn post_delay_api(
    req: HttpRequest,
    data: Bytes,
    path: Path<u64>,
) -> Json<ApiOutput> {
    delay_anything(req, data, path).await
}

#[utoipa::path(
    tag = TAG,
    patch,
    path = "/delay/{delay}",
    responses(
        (status=200, description="Returns a delayed response (max of 10 seconds).", body=ApiOutput)
    ),
    params(
        ("delay"=u64, Path, description="Delay in seconds.")
    )
)]
pub(crate) async fn patch_delay_api(
    req: HttpRequest,
    data: Bytes,
    path: Path<u64>,
) -> Json<ApiOutput> {
    delay_anything(req, data, path).await
}

#[utoipa::path(
    tag = TAG,
    put,
    path = "/delay/{delay}",
    responses(
        (status=200, description="Returns a delayed response (max of 10 seconds).", body=ApiOutput)
    ),
    params(
        ("delay"=u64, Path, description="Delay in seconds.")
    )
)]
pub(crate) async fn put_delay_api(
    req: HttpRequest,
    data: Bytes,
    path: Path<u64>,
) -> Json<ApiOutput> {
    delay_anything(req, data, path).await
}

async fn delay_anything(req: HttpRequest, data: Bytes, path: Path<u64>) -> Json<ApiOutput> {
    let delay = path.into_inner();
    sleep(Duration::from_secs(delay)).await;
    anything(req, data).await
}
