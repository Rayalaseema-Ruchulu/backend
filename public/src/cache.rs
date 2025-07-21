use axum::{extract::Request, middleware::Next, response::Response};
use http::header::CACHE_CONTROL;

use crate::error::ApiResult;

// TODO add etag caching
// pub async fn generate_etag(response: Response) -> Result<Response> {
//     let hasher = DigestStream::new(DigestStreamAlgorithm::Sha1);
//     let http_response = HttpResponse::try_from(response)?;
//     let js_response = response_to_wasm(http_response)?;

//     let _ = js_response.body().unwrap().pipe_to(hasher.raw());
//     let hash = hasher
//         .digest()
//         .await?
//         .to_vec()
//         .iter()
//         .map(|byte| format!("{:02x}", byte))
//         .collect::<String>();

//     todo!()
// }

pub(crate) async fn cache_control_middleware(request: Request, next: Next) -> ApiResult<Response> {
    let mut response = next.run(request).await;

    response
        .headers_mut()
        .insert(CACHE_CONTROL, "public, max-age=300".parse().unwrap());

    Ok(response)
}
