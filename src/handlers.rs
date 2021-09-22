use trillium::{Body, Conn, KnownHeaderName, Status};

pub async fn remove_server_response_header(mut conn: Conn) -> Conn {
    conn.headers_mut().remove("Server");
    conn
}

pub async fn ensure_trailing_slash(conn: Conn) -> Conn {
    let path = conn.path().to_owned();
    if path.ends_with("/") {
        conn
    } else {
        conn.with_status(Status::PermanentRedirect)
            .with_header(KnownHeaderName::Location, format!("{}/", path))
            .halt()
    }
}

/// requires async-fs (smol) crate
pub async fn body_from_path<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Body> {
    use async_fs::{self as fs, File};
    let file = File::open(&path).await?;
    let len = fs::metadata(&path).await?.len();
    Ok(Body::new_streaming(file, Some(len)))
}

/// requires mime_guess crate
pub async fn body_from_path_with_mime<P: AsRef<std::path::Path>>(
    path: P,
) -> std::io::Result<(Body, String)> {
    Ok((
        body_from_path(&path).await?,
        mime_guess::from_path(&path)
            .first_or_octet_stream()
            .to_string(),
    ))
}
