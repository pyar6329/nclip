use super::*;

#[handler]
#[tracing::instrument]
pub(super) async fn healthz(res: &mut Response) {
    info!("healthz was called");
    let response = ResponseCommon {
        status: StatusCode::OK.as_u16(),
        content: "health status is OK",
    };
    res.render(Json(response));
}
