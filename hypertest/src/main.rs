use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body:: { Bytes, Body };
//use hyper::body;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;


use log::{debug, info, trace};

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {

    //trace!("Incoming request is: {:?}",  );
    let random_byte = rand::random::<u8>();
    debug!("Generated value is: {}", random_byte);
    //Response::new(Body::from(random_byte.to_string()))

    Ok(Response::new(Full::new(Bytes::from(random_byte.to_string()))))
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

