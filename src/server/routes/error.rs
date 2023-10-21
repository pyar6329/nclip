use super::*;

#[handler]
pub(super) async fn handle_error(
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
