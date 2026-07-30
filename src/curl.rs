//! Parsing y serialización mínimos para un cliente HTTP educativo.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpUrl {
    pub host: String,
    pub port: u16,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

pub fn parse_http_url(input: &str) -> Result<HttpUrl, String> {
    let authority_and_path = input
        .strip_prefix("http://")
        .ok_or_else(|| "solo se admite el esquema http".to_owned())?;
    let (authority, path) = authority_and_path.split_once('/').map_or_else(
        || (authority_and_path, "/".to_owned()),
        |(host, rest)| (host, format!("/{rest}")),
    );
    if authority.is_empty() {
        return Err("la URL requiere un host".into());
    }
    let (host, port) = authority.rsplit_once(':').map_or_else(
        || Ok((authority, 80)),
        |(host, port)| {
            port.parse()
                .map(|port| (host, port))
                .map_err(|_| "el puerto no es válido".to_owned())
        },
    )?;
    if host.is_empty() {
        return Err("la URL requiere un host".into());
    }
    Ok(HttpUrl {
        host: host.into(),
        port,
        path,
    })
}

pub fn build_get_request(url: &HttpUrl) -> Vec<u8> {
    format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path, url.host
    )
    .into_bytes()
}

pub fn parse_response(input: &[u8]) -> Result<HttpResponse, String> {
    let boundary = input
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| "la respuesta HTTP está incompleta".to_owned())?;
    let header_text = std::str::from_utf8(&input[..boundary])
        .map_err(|_| "los encabezados no son UTF-8".to_owned())?;
    let mut lines = header_text.split("\r\n");
    let status_line = lines
        .next()
        .ok_or_else(|| "falta la línea de estado".to_owned())?;
    let status = status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| "la línea de estado es inválida".to_owned())?
        .parse::<u16>()
        .map_err(|_| "el estado HTTP es inválido".to_owned())?;
    if !(100..=599).contains(&status) {
        return Err("el estado HTTP está fuera de rango".into());
    }
    let headers = lines
        .map(|line| {
            let (name, value) = line
                .split_once(':')
                .ok_or_else(|| "el encabezado es inválido".to_owned())?;
            Ok((name.to_ascii_lowercase(), value.trim().to_owned()))
        })
        .collect::<Result<_, String>>()?;
    Ok(HttpResponse {
        status,
        headers,
        body: input[boundary + 4..].to_vec(),
    })
}
