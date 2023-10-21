use anyhow::Result;
use salvo::catcher::Catcher;
use salvo::conn::quinn::{QuinnAcceptor, QuinnListener};
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::conn::tcp::TcpAcceptor;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tracing::info;

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

fn init_tracing() {
    tracing_subscriber::fmt()
        .json()
        .with_current_span(false)
        .flatten_event(true)
        .with_span_list(true)
        .init();
}

fn create_router() -> Router {
    let health = Router::with_path("healthz").get(healthz);
    let clipboards = Router::with_path("clipboards")
        .get(get_clipboard)
        .post(post_clipboard);
    Router::new().get(healthz).push(health).push(clipboards)
}

fn create_service(router: Router) -> Service {
    Service::new(router).catcher(Catcher::default().hoop(handle_error))
}

async fn http1_server(port: &u16) -> TcpAcceptor {
    TcpListener::new(("0.0.0.0", *port)).bind().await
}

// async fn http3_server<A, B, C>(port: &u16) -> QuinnAcceptor<A, B, C> {
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

#[handler]
#[tracing::instrument]
async fn healthz(res: &mut Response) {
    info!("healthz was called");
    let response = ResponseClipboard {
        status: 200,
        content: "health status is OK",
    };
    res.render(Json(response));
}

#[handler]
async fn get_clipboard(res: &mut Response, ctrl: &mut FlowCtrl) {
    ctrl.skip_rest(); // skip middleware
    info!("GET /clipboards was called");

    let response = ResponseClipboard {
        status: 200,
        content: "aaaaaaaaaaaaaaaaaaaaaa",
    };
    res.render(Json(response));
}

#[handler]
async fn post_clipboard<'a>(body: RequestClipboard<'a>, res: &mut Response, ctrl: &mut FlowCtrl) {
    ctrl.skip_rest(); // skip middleware
    info!("ooooooooo {}", &body.content);

    let response = ResponseClipboard {
        status: 200,
        content: &format!("hogehoge {}", &body.content),
    };
    res.render(Json(response));
}

#[derive(Deserialize, Extractible, Debug)]
#[salvo(extract(default_source(from = "body", format = "json")))]
struct RequestClipboard<'a> {
    content: &'a str,
}

#[derive(Debug, Serialize)]
struct ResponseClipboard<'a> {
    status: u16,
    content: &'a str,
}

#[handler]
async fn handle_error(
    &self,
    _req: &Request,
    _depot: &Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let status = res.status_code.unwrap_or_default();
    let content = match status {
        StatusCode::BAD_REQUEST => "invalid parameter is set",
        StatusCode::INTERNAL_SERVER_ERROR => "internal error occurred",
        s if s.is_client_error() => "4xx error occurred",
        s if s.is_server_error() => "5xx error occurred",
        _ => "something went wrong",
    };
    let response = ResponseClipboard {
        status: status.as_u16(),
        content,
    };

    if status.is_client_error() || status.is_server_error() {
        res.status_code(status);
        res.render(Json(response));
        ctrl.skip_rest();
    }
}
