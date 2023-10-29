use axum::ServiceExt;
use hyper::{service::service_fn, Body, Request, Response, Server};
use std::env;
use std::net::SocketAddr;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let service = ServiceBuilder::new()
        .service(service_fn(forward_request))
        .into_make_service();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let server = Server::bind(&addr).serve(service);

    println!("Listening on {}", addr);

    server.await.unwrap()
}

async fn forward_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Get the environment variable
    let proxy_host = env::var("PROXY_HOST").unwrap();

    // Set target API URL
    let target_url = format!("https://{proxy_host}{}", req.uri());

    // Prepare the request using reqwest
    let client = reqwest::Client::new();
    let req_method = req.method().clone();
    let builder = client.request(req_method, &target_url);

    // Remove the host header and proxy the rest
    let mut headers = req.headers().clone();
    headers.remove("host");
    let builder = builder.headers(headers);

    // Forward body
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let builder = builder.body(body_bytes);

    // Log the proxying and entire request headers and body
    println!("Proxying request to {}, content: {:?}", target_url, builder);

    // Make the request
    let res = builder.send().await.unwrap();

    // Prepare response
    let mut response = Response::builder().status(res.status());

    // Forward headers back to client
    for (name, value) in res.headers().iter() {
        response = response.header(name, value);
    }

    // Forward body back to client
    let res_bytes = res.bytes().await.unwrap();
    let response = response.body(Body::from(res_bytes)).unwrap();

    Ok(response)
}
