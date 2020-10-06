use hyper::{
    Request,
    Body,
    StatusCode,
    Response,
    Method,
};
use log::debug;
use super::Result;
use super::handlers::*;

pub async fn router(request: Request<Body>) -> Result<Response<Body>> {
    match(request.method(), request.uri().path()) {
	(&Method::GET, search) if search.starts_with("/hello") => {
            debug!("/hello triggered!");
	    hello_world(request).await

        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}


#[cfg(test)]
mod tests {
    use hyper::{
        Body,
        StatusCode,
    };
    use mockito;

    #[tokio::test]
    async fn test_router() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let addr = &mockito::server_address();
        // let _m = mockito::mock("GET", "/asd")
        //     .with_status(200)
        //     .with_header("content-type", "application/json")
        //     .with_body(r#"{"test": "data"}"#)
        //     .create();

        let req_addr =
            "http://".to_string() + &addr.to_string() + "/hello";

        let req = hyper::Request::builder()
            .method("GET")
            .uri(req_addr)
            .body(Body::empty())
            .expect("request builder");

        let res = super::router(req).await?;
        let body = hyper::body::to_bytes(res.into_body()).await?;
        assert_eq!(body, r#"{"message": "Hello Universe" }"#);

        let req = hyper::Request::builder()
            .method("GET")
            .uri("http://".to_string() + &addr.to_string() + "/does_not_exist")
            .body(Body::empty())
            .expect("request builder");
        let res = super::router(req).await?;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        Ok(())
    }
}
