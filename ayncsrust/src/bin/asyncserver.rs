use hyper:: {
    service::{make_service_fn, service_fn},
    Body,
    Client,
    Request,
    Response,
    Server,
    Uri,
};
use std::net::SocketAddr;
use std::fs;

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let contents = fs::read_to_string("/home/antash/Desktop/website/current_elec_intro.html")
        .unwrap();
    let url_str = "http://www.google.com";
    let url = url_str.parse::<Uri>().expect("failed to parse URl");
    let res = Client::new().get(url).await?;
    Ok(Response::new(Body::from(contents)))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr)
        .serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(serve_req))
        }));

    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run_server(addr).await;
}