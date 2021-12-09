use rblog::app::app;

use trillium_testing::prelude::*;

#[test]
fn should_render_homepage() {
    let handler = app();
    assert_ok!(get("/").on(&handler));
}
