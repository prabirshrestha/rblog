use trillium::{Conn, KnownHeaderName, Status};

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
