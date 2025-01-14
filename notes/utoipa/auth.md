# auth

在 openapi 中，集成接口认证。

首先，我们需要自定义安全认证的插件，例如 `SecurityAddon`，并为其实现 `Modify` trait，参考以下代码：

```rust
#[derive(OpenApi)]
#[openapi(
    ...
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
```

然后，我们可以在使用 `#[utoipa::path]` 宏时，为其添加安全认证，参考以下代码：

```rust
#[utoipa::path(
    tag = TAG,
    get,
    path = "/bearer",
    responses(
        (status=200, description="Sucessful authentication."),
        (status=401, description="Unsuccessful authentication.")
    ),
    security(
        (),
        ("bearerAuth" = [])
    )
)]
...
```
