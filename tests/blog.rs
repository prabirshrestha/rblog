use rblog::app::App;
use salvo::{
    http::StatusCode,
    test::{ResponseExt, TestClient},
};

async fn app() -> App {
    App::from_config_file("blog.yaml").await.unwrap()
}

#[tokio::test]
async fn renders_homepage_and_post() {
    let service = app().await.service();
    for path in ["/", "/posts/welcome/"] {
        let mut response = TestClient::get(format!("http://localhost{path}"))
            .send(&service)
            .await;
        assert_eq!(response.status_code, Some(StatusCode::OK));
        assert!(
            response.headers()["content-type"]
                .to_str()
                .unwrap()
                .starts_with("text/html")
        );
        assert!(response.take_string().await.unwrap().contains("Welcome"));
    }
}

#[tokio::test]
async fn serves_healthcheck_and_robots() {
    let service = app().await.service();
    let mut response = TestClient::get("http://localhost/healthcheck")
        .send(&service)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    assert_eq!(response.take_string().await.unwrap(), "OK");

    let mut response = TestClient::get("http://localhost/robots.txt")
        .send(&service)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    assert!(
        response
            .take_string()
            .await
            .unwrap()
            .contains("Disallow: /healthcheck")
    );
}

#[tokio::test]
async fn serves_rss() {
    let service = app().await.service();
    let mut response = TestClient::get("http://localhost/rss").send(&service).await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    assert_eq!(response.headers()["content-type"], "application/rss+xml");
    let body = response.take_string().await.unwrap();
    assert!(body.contains("<rss"));
    assert!(body.contains("Welcome"));
}

#[tokio::test]
async fn redirects_mixed_case_post_and_attachment() {
    let service = app().await.service();
    for (path, status, location) in [
        (
            "/posts/Welcome/",
            StatusCode::MOVED_PERMANENTLY,
            "/posts/welcome/",
        ),
        (
            "/posts/Welcome/attachment.txt",
            StatusCode::SEE_OTHER,
            "/posts/welcome/attachment.txt",
        ),
    ] {
        let response = TestClient::get(format!("http://localhost{path}"))
            .send(&service)
            .await;
        assert_eq!(response.status_code, Some(status));
        assert_eq!(response.headers()["location"], location);
    }
}

#[tokio::test]
async fn serves_attachment_and_byte_range() {
    let service = app().await.service();
    let expected = std::fs::read("posts/welcome/attachment.txt").unwrap();
    let mut response = TestClient::get("http://localhost/posts/welcome/attachment.txt")
        .send(&service)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    assert_eq!(
        response.take_bytes(None).await.unwrap().as_ref(),
        expected.as_slice()
    );

    let mut response = TestClient::get("http://localhost/posts/welcome/attachment.txt")
        .add_header("range", "bytes=0-3", true)
        .send(&service)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::PARTIAL_CONTENT));
    assert_eq!(
        response.headers()["content-range"],
        format!("bytes 0-3/{}", expected.len())
    );
    assert_eq!(
        response.take_bytes(None).await.unwrap().as_ref(),
        &expected[..4]
    );
}

#[tokio::test]
async fn serves_embedded_assets() {
    let service = app().await.service();
    let mut home = TestClient::get("http://localhost/").send(&service).await;
    let body = home.take_string().await.unwrap();
    let asset = body
        .split('"')
        .find(|value| value.starts_with("/assets/") && value.ends_with(".css"))
        .unwrap();
    let mut response = TestClient::get(format!("http://localhost{asset}"))
        .send(&service)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    assert!(
        response.headers()["content-type"]
            .to_str()
            .unwrap()
            .starts_with("text/css")
    );
    assert_eq!(response.headers()["cache-control"], "max-age=31536000");
    assert!(!response.take_bytes(None).await.unwrap().is_empty());
}

#[tokio::test]
async fn returns_not_found_for_missing_resources() {
    let service = app().await.service();
    for path in ["/missing", "/posts/missing/", "/posts/welcome/missing.txt"] {
        let response = TestClient::get(format!("http://localhost{path}"))
            .send(&service)
            .await;
        assert_eq!(response.status_code, Some(StatusCode::NOT_FOUND));
    }
}

#[tokio::test]
async fn head_has_no_body() {
    let service = app().await.service();
    let mut response = TestClient::head("http://localhost/posts/welcome/")
        .send(&service)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    assert!(response.take_bytes(None).await.unwrap().is_empty());
}
