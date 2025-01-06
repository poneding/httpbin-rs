use actix_web::{
    web::{delete, get, patch, post, put, resource, Bytes, Json, ServiceConfig},
    HttpRequest,
};
use serde::Serialize;
use utoipa::ToSchema;
use std::collections::HashMap;

#[derive(Serialize, ToSchema)]
pub(super) struct ApiOutput {
    args: Option<HashMap<String, String>>,
    data: String,
    files: Option<Vec<String>>,
    form: Option<Vec<String>>,
    headers: HashMap<String, String>,
    json: Option<String>,
    origin: String,
    url: String,
    method: String,
}

pub(crate) fn api(cfg: &mut ServiceConfig) {
    cfg.service(resource("/delete").route(delete().to(delete_api)))
    .service(resource("/get").route(get().to(get_api)))
    .service(resource("/patch").route(patch().to(patch_api)))
    .service(resource("/post").route(post().to(post_api)))
    .service(resource("/put").route(put().to(put_api)));
}

#[utoipa::path(
    get, 
    path = "/delete",
    responses(
        (status=200, body=ApiOutput)
    )
)]
pub(crate) async fn delete_api(req: HttpRequest, data: Bytes) -> Json<ApiOutput> {
    anything(req, data).await
}

#[utoipa::path(
    get, 
    path = "/get",
    responses(
        (status=200, body=ApiOutput)
    )
)]
pub(crate) async fn get_api(req: HttpRequest, data: Bytes) -> Json<ApiOutput> {
    anything(req, data).await
}

#[utoipa::path(
    get, 
    path = "/patch",
    responses(
        (status=200, body=ApiOutput)
    )
)]
pub(crate) async fn patch_api(req: HttpRequest, data: Bytes) -> Json<ApiOutput> {
    anything(req, data).await
}

#[utoipa::path(
    get, 
    path = "/post",
    responses(
        (status=200, body=ApiOutput)
    )
)]
pub(crate) async fn post_api(req: HttpRequest, data: Bytes) -> Json<ApiOutput> {
    anything(req, data).await
}

#[utoipa::path(
    get, 
    path = "/put",
    responses(
        (status=200, body=ApiOutput)
    )
)]
pub(crate) async fn put_api(req: HttpRequest, data: Bytes) -> Json<ApiOutput> {
    anything(req, data).await
}

async fn anything(req: HttpRequest, data: Bytes) -> Json<ApiOutput> {
    let headers = req
    .headers()
    .iter()
    .map(|x| {
        (
            x.0.to_string(),
            x.1.to_str().unwrap_or_default().to_string(),
        )
    })
    .collect();

    let mut args = HashMap::new();
        if !req.query_string().is_empty(){
        for arg in req.query_string().split('&'){
            let mut kv = arg.split('=');
            let key = kv.next().unwrap_or_default().to_string();
            let mut value = kv.next().unwrap_or_default().to_string();
            if let Some(v) = args.get(&key){
                value = format!("{},{}", v, value);
            }
            args.insert(key, value);
        }
    }

    Json(ApiOutput {
        args: args.is_empty().then_some(args),
        data: String::from_utf8(data.to_vec()).unwrap_or_default(),
        files: None,
        form: None,
        headers,
        json: None,
        origin: req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or_default()
            .to_string(),
        url: req.full_url().to_string(),
        method: req.method().to_string(),
    })
}
