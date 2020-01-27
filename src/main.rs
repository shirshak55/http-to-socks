use std::convert::Infallible;
use std::net::{SocketAddr, ToSocketAddrs};

use futures_util::future::try_join;

use hyper::service::{make_service_fn, service_fn};
use hyper::upgrade::Upgraded;
use hyper::{Body, Client, Method, Request, Response, Server};

use async_socks5::{connect as connectSock, Auth};
use tokio::net::TcpStream;

use std::env::var as get_env;

type HttpClient = Client<hyper::client::HttpConnector>;

#[tokio::main]
async fn main() {
    let server_addr = get_env("SERVER_SOADDR")
        .expect("Please provide server address to bind using environment variable")
        .to_socket_addrs()
        .expect("Unable to parse connect header")
        .next()
        .expect("No socket address found");

    let addr = SocketAddr::from(server_addr);
    let client = HttpClient::new();

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if Method::CONNECT == req.method() {
        if let Some(addr) = host_addr(req.uri()) {
            tokio::task::spawn(async move {
                match req.into_body().on_upgrade().await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, addr).await {
                            eprintln!("server io error: {}", e);
                        };
                    }
                    Err(e) => eprintln!("upgrade error: {}", e),
                }
            });

            Ok(Response::new(Body::empty()))
        } else {
            eprintln!("CONNECT host is not socket addr: {:?}", req.uri());
            let mut resp = Response::new(Body::from("CONNECT must be to a socket address"));
            *resp.status_mut() = http::StatusCode::BAD_REQUEST;

            Ok(resp)
        }
    } else {
        dbg!("reqaaaa", &req);
        client.request(req).await
    }
}

fn host_addr(uri: &http::Uri) -> Option<SocketAddr> {
    uri.authority().and_then(|auth| {
        auth.as_str()
            .to_socket_addrs()
            .expect("Unable to parse connect header")
            .next()
    })
}

// Create a TCP connection to host:port, build a tunnel between the connection and
// the upgraded connection
async fn tunnel(upgraded: Upgraded, addr: SocketAddr) -> std::io::Result<()> {
    // Connect to remote server
    let socket_addr = get_env("SOCKS_SOADDR").expect("Please provide socks upstream addr");

    let mut server = TcpStream::connect(socket_addr).await?;

    let sock_connection = if let Ok(username) = get_env("SOCKS_USERNAME") {
        let password = get_env("SOCKS_PASSWORD").expect("Please provide password");
        connectSock(
            &mut server,
            (addr.ip(), addr.port()),
            Some(Auth { username, password }),
        )
    } else {
        // we assume there is no authentication needed
        connectSock(&mut server, (addr.ip(), addr.port()), None)
    };

    sock_connection.await.expect("Unable to connect to socks");

    // Proxying data
    let amounts = {
        let (mut server_rd, mut server_wr) = server.split();
        let (mut client_rd, mut client_wr) = tokio::io::split(upgraded);

        let client_to_server = tokio::io::copy(&mut client_rd, &mut server_wr);
        let server_to_client = tokio::io::copy(&mut server_rd, &mut client_wr);

        try_join(client_to_server, server_to_client).await
    };

    if let Err(e) = amounts {
        println!("tunnel error: {}", e);
    }

    Ok(())
}
