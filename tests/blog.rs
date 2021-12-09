use rblog::app::app;
use trillium::KnownHeaderName;
use trillium_testing::prelude::*;

#[test]
fn should_render_homepage() {
    let handler = app();
    assert_ok!(get("/").on(&handler));
    assert_headers!(
        get("/").on(&handler),
        "content-type" => "text/html; charset=utf-8"
    );
}

#[test]
fn should_compress_html() {
    let handler = app();
    assert_headers!(
        get("/")
            .with_request_header(KnownHeaderName::AcceptEncoding, "gzip, deflate, br")
            .on(&handler),
        "content-encoding" => "br"
    );

    assert_headers!(
        get("/")
            .with_request_header(KnownHeaderName::AcceptEncoding, "gzip")
            .on(&handler),
        "content-encoding" => "gzip"
    );
}
