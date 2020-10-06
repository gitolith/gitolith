use hyper::{
    header,
    Request,
    Body,
    Response,
};
use log::debug;
use super::Result;

pub async fn hello_world(request: Request<Body>) -> Result<Response<Body>> {
    let data = r#"{"message": "Hello Universe" }"#;
    let response = Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(data))?;
    let parts = request.uri().path_and_query();
    debug!("Request data: {:#?}", &parts);

    Ok(response)
}
