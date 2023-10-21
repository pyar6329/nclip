use super::*;

#[handler]
pub(super) async fn get_clipboard(res: &mut Response, ctrl: &mut FlowCtrl) {
    ctrl.skip_rest(); // skip middleware
    info!("GET /clipboards was called");

    let response = ResponseClipboard {
        status: 200,
        content: "aaaaaaaaaaaaaaaaaaaaaa",
    };
    res.render(Json(response));
}

#[handler]
pub(super) async fn post_clipboard<'a>(
    body: RequestClipboard<'a>,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    ctrl.skip_rest(); // skip middleware
    info!("ooooooooo {}", &body.content);

    let response = ResponseClipboard {
        status: 200,
        content: &format!("hogehoge {}", &body.content),
    };
    res.render(Json(response));
}
