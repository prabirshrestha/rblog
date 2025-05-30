use rblog::app_config::{AppConfig, Facebook};

#[test]
fn test_facebook_config_creation() {
    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: "8080".to_string(),
        title: "Test Blog".to_string(),
        root: "https://test.com".to_string(),
        page_size: None,
        enable_drafts: None,
        posts_dir: "posts".to_string(),
        github: None,
        mastodon: None,
        twitter: None,
        facebook: Some(Facebook {
            app_id: "123456789".to_string(),
        }),
        disqus: None,
        giscus: None,
        google_analytics: None,
        syntax_highlight: None,
    };

    assert!(config.facebook.is_some());
    assert_eq!(config.facebook.unwrap().app_id, "123456789");
}

#[test]
fn test_twitter_config_exists() {
    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: "8080".to_string(),
        title: "Test Blog".to_string(),
        root: "https://test.com".to_string(),
        page_size: None,
        enable_drafts: None,
        posts_dir: "posts".to_string(),
        github: None,
        mastodon: None,
        twitter: Some("testuser".to_string()),
        facebook: None,
        disqus: None,
        giscus: None,
        google_analytics: None,
        syntax_highlight: None,
    };

    assert!(config.twitter.is_some());
    assert_eq!(config.twitter.unwrap(), "testuser");
}

#[test]
fn test_facebook_yaml_parsing() {
    let config = AppConfig::from_config_file("/tmp/test_config.yaml");
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(config.facebook.is_some());
    assert_eq!(config.facebook.unwrap().app_id, "test_app_id_123");
}