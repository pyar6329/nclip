use super::*;

#[handler]
#[tracing::instrument]
pub(super) async fn healthz(res: &mut Response) {
    info!("healthz was called");
    let response = ResponseClipboard {
        status: 200,
        content: "health status is OK",
    };
    res.render(Json(response));
}
