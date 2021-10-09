use trillium::{Conn, KnownHeaderName, Status};

pub async fn remove_server_response_header(mut conn: Conn) -> Conn {
    conn.headers_mut().remove("Server");
    conn
}

pub async fn ensure_trailing_slash(conn: Conn) -> Conn {
    let path = conn.path().to_owned();
    if path.ends_with('/') {
        conn
    } else {
        conn.with_status(Status::PermanentRedirect)
            .with_header(KnownHeaderName::Location, format!("{}/", path))
            .halt()
    }
}
