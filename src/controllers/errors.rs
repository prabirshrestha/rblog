use salvo::prelude::*;

use crate::{templates, utils::render::RenderExt};

#[handler]
pub async fn not_found(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
    ctrl: &mut FlowCtrl,
) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        match res.render_html(|o| templates::errors::not_found_html(o)) {
            Ok(_) => {}
            Err(_) => {
                ctrl.call_next(req, depot, res).await;
            }
        }
    } else {
        ctrl.call_next(req, depot, res).await;
    }
}
