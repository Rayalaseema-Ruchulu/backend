use worker::{Request, Response, Result, RouteContext};

pub async fn send_to_common_backend(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    ctx.service("common-backend")?.fetch_request(req).await
}
