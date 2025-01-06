use actix_web::{
    web::{self, get, resource, ServiceConfig},
    HttpRequest, HttpResponse, Result,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde_json::json;

const TAG: &str = "Auth";

pub(crate) fn api(cfg: &mut ServiceConfig) {
    cfg.service(resource("/basic-auth/{user}/{passwd}").route(get().to(basic_auth_api)));
}

#[utoipa::path(
    tag = TAG,
    get,
    params(
        ("user", description="user name"),
        ("passwd", description="password"),
    ),
    path = "/basic-auth/{user}/{passwd}",
    responses(
        (status=200, description="TSucessful authentication."),
        (status=401, description="Unsuccessful authentication.")
    )
)]
pub(crate) async fn basic_auth_api(
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let (user, passwd) = path.into_inner();

    if let Some(auth) = req.headers().get("Authorization") {
        // base64 decode
        let auth = auth.to_str().unwrap();
        if auth.starts_with("Basic ") {
            let auth = auth.trim_start_matches("Basic ");
            let decode = BASE64_STANDARD.decode(auth).unwrap();
            let decode_str = String::from_utf8(decode).unwrap();
            let (decoded_user, decoded_passwd) = decode_str.split_once(':').unwrap();

            if user == decoded_user && passwd == decoded_passwd {
                return Ok(HttpResponse::Ok().json(json!({
                    "authenticated": true,
                    "user": user,
                })));
            }
        }
    }

    Ok(HttpResponse::Unauthorized()
        .insert_header(("WWW-Authenticate", "Basic realm=\"Fake Realm\""))
        .body("Unsuccessful authentication."))
}
