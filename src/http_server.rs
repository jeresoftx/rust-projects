//! Router HTTP exacto para un servidor educativo.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

#[derive(Debug, Default)]
pub struct Router {
    routes: Vec<(String, String, String)>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn route(mut self, method: &str, path: &str, body: &str) -> Self {
        self.routes.push((method.into(), path.into(), body.into()));
        self
    }
    pub fn handle(&self, request: Request) -> Response {
        if let Some((_, _, body)) = self
            .routes
            .iter()
            .find(|(method, path, _)| method == &request.method && path == &request.path)
        {
            return Response {
                status: 200,
                body: body.clone(),
            };
        }
        let status = if self.routes.iter().any(|(_, path, _)| path == &request.path) {
            405
        } else {
            404
        };
        Response {
            status,
            body: String::new(),
        }
    }
}

pub fn parse_request(bytes: &[u8]) -> Result<Request, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "la request no es UTF-8".to_owned())?;
    let line = text
        .split("\r\n")
        .next()
        .ok_or_else(|| "falta la línea inicial".to_owned())?;
    let mut parts = line.split_whitespace();
    let (Some(method), Some(path), Some(version), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err("la línea inicial es inválida".into());
    };
    if version != "HTTP/1.1" || !path.starts_with('/') || path.contains(['\r', '\n']) {
        return Err("la request es inválida".into());
    }
    Ok(Request {
        method: method.into(),
        path: path.into(),
    })
}

pub fn serialize_response(response: &Response) -> Vec<u8> {
    let reason = match response.status {
        200 => "OK",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Bad Request",
    };
    format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        response.status,
        reason,
        response.body.len(),
        response.body
    )
    .into_bytes()
}
