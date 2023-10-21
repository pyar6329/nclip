use super::*;
use crate::utils::*;

#[handler]
// #[tracing::instrument(skip(_req, _depot, ctrl), fields(body = res.as_serde()))]
#[tracing::instrument(skip(_req, _depot, ctrl))]
pub(super) async fn handle_error(
    &self,
    _req: &Request,
    _depot: &Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let status = res.status_code.unwrap_or_default();
    let content = match status {
        StatusCode::BAD_REQUEST => warn_msg("invalid parameter is set"),
        StatusCode::INTERNAL_SERVER_ERROR => warn_msg("internal error occurred"),
        s if s.is_client_error() => warn_msg("4xx error occurred"),
        s if s.is_server_error() => err_msg("5xx error occurred"),
        s if s.is_informational() || s.is_success() || s.is_redirection() => String::default(),
        _ => err_msg("something went wrong"),
    };
    let response = ResponseCommon {
        status: status.as_u16(),
        content: &content,
    };

    if status.is_client_error() || status.is_server_error() {
        res.status_code(status);
        res.render(Json(response));
        ctrl.skip_rest();
    }
}
