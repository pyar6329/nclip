use super::*;
use crate::utils::*;
use anyhow::Result;

#[handler]
pub(super) async fn get_clipboard(
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<(), GetClipboardError> {
    ctrl.skip_rest(); // skip middleware
    info!("GET /clipboards was called");

    let content = Clipboard::get_clipboard().map_err(|e| {
        err_msg(e.to_string());
        GetClipboardError
    })?;

    let response = ResponseClipboard {
        status: StatusCode::OK.as_u16(),
        content: &content,
    };
    res.render(Json(response));

    Ok(())
}

#[handler]
pub(super) async fn post_clipboard<'a>(
    body: RequestClipboard<'a>,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<(), PostClipboardError> {
    ctrl.skip_rest(); // skip middleware
    info!("POST /clipboards was called");

    let compressed_content = Zstd::from(body.content);
    Clipboard::set_clipboard(&compressed_content).map_err(|e| {
        err_msg(e.to_string());
        PostClipboardError
    })?;

    let response = ResponseClipboard {
        status: StatusCode::CREATED.as_u16(),
        content: body.content,
    };
    res.render(Json(response));

    Ok(())
}

struct GetClipboardError;
#[async_trait]
impl Writer for GetClipboardError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;
        let error_message = err_msg("cannot get clipboard");
        let response = ResponseCommon {
            status: status_code.as_u16(),
            content: &error_message,
        };

        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(response));
    }
}

struct PostClipboardError;
#[async_trait]
impl Writer for PostClipboardError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;
        let error_message = err_msg("cannot set clipboard");
        let response = ResponseCommon {
            status: status_code.as_u16(),
            content: &error_message,
        };

        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(response));
    }
}
