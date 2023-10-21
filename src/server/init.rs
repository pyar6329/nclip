use salvo::conn::tcp::{TcpAcceptor, TcpListener};
use salvo::conn::Listener;
// use salvo::catcher::Catcher;
// use salvo::conn::quinn::{QuinnAcceptor, QuinnListener};
// use salvo::conn::rustls::{Keycert, RustlsConfig};
// use salvo::conn::tcp::TcpAcceptor;
// use salvo::prelude::*;

pub(super) fn init_tracing() {
    tracing_subscriber::fmt()
        .json()
        .with_current_span(false)
        .flatten_event(true)
        .with_span_list(true)
        .init();
}

pub(super) async fn http1_server(port: &u16) -> TcpAcceptor {
    TcpListener::new(("0.0.0.0", *port)).bind().await
}

// pub(super) async fn http3_server<A, B, C>(port: &u16) -> QuinnAcceptor<A, B, C> {
//     let cert = include_bytes!("../certs/cert.pem").to_vec();
//     let key = include_bytes!("../certs/key.pem").to_vec();
//
//     let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));
//     let listener = TcpListener::new(("0.0.0.0", *port)).rustls(config.clone());
//
//     let acceptor = QuinnListener::new(config, ("0.0.0.0", *port))
//         .join(listener)
//         .bind()
//         .await;
//
//     acceptor
//     // Server::new(acceptor).serve(router).await
// }
