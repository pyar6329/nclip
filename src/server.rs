mod init;
mod requests;
mod routes;

use init::*;
use routes::*;
use salvo::Server;
use tokio::sync::oneshot;
// use tracing::info;

#[tokio::main]
pub async fn run() {
    init_tracing();

    let router = create_router();
    let service = create_service(router);
    let port = 5800;

    // http3_server(&port, router).await;
    let acceptor = http1_server(&port).await;

    let (tx, rx) = oneshot::channel();
    let server = Server::new(acceptor).serve_with_graceful_shutdown(
        service,
        async {
            rx.await.ok();
        },
        None,
    );

    tokio::task::spawn(server);

    let quit_key = tokio::signal::ctrl_c().await;
    if quit_key.is_ok() {
        let _ = tx.send(());
    }
}
