mod clipboard;
mod error;
mod health;

use super::requests::*;
use clipboard::*;
use error::*;
use health::*;
use salvo::catcher::Catcher;
use salvo::prelude::*;
use tracing::*;

pub(super) fn create_router() -> Router {
    let health = Router::with_path("healthz").get(healthz);
    let clipboards = Router::with_path("clipboards")
        .get(get_clipboard)
        .post(post_clipboard);
    Router::new().get(healthz).push(health).push(clipboards)
}

pub(super) fn create_service(router: Router) -> Service {
    Service::new(router).catcher(Catcher::default().hoop(handle_error))
}
