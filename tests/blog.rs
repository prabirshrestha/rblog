/*
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

    // TODO: comment for now. https://github.com/trillium-rs/trillium/issues/145
    // assert_headers!(
    //     get("/")
    //         .with_request_header(KnownHeaderName::AcceptEncoding, "")
    //         .on(&handler),
    //     "content-encoding" => None
    // );
}

#[test]
fn should_set_request_id_header() {
    let handler = app();

    let conn = get("/").run(&handler);
    let x_request_id_header = conn.inner().response_headers().get_str("x-request-id");
    assert!(x_request_id_header.is_some());

    let conn = get("/posts/welcome/").run(&handler);
    let x_request_id_header = conn.inner().response_headers().get_str("x-request-id");
    assert!(x_request_id_header.is_some());

    let conn = get("/posts/welcome/attachment.txt").run(&handler);
    let x_request_id_header = conn.inner().response_headers().get_str("x-request-id");
    assert!(x_request_id_header.is_some());
}

#[test]
fn should_add_trailing_slash_for_posts() {
    let handler = app();

    let conn = get("/posts/welcome").run(&handler);
    assert_status!(conn, 308);
    assert_headers!(conn, "Location" => "/posts/welcome/");

    let conn = get("/posts/Welcome").run(&handler);
    assert_status!(conn, 308);
    assert_headers!(conn, "Location" => "/posts/Welcome/");
}

#[test]
fn should_return_premanent_slug_mismatch_for_get_post() {
    let handler = app();

    let conn = get("/posts/Welcome/").run(&handler);
    assert_status!(conn, 308);
    assert_headers!(conn, "Location" => "/posts/welcome/");
}

#[test]
fn should_return_premanent_slug_mismatch_for_attachments() {
    let handler = app();

    let conn = get("/posts/Welcome/welcome.txt").run(&handler);
    assert_status!(conn, 308);
    assert_headers!(conn, "Location" => "/posts/welcome/welcome.txt");
}
*/
