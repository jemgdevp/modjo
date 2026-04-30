// Módulo para manejar la lógica de envío de requests HTTP.
//
// Este módulo contiene la función `send_request` que envía un request HTTP
// y devuelve un `ResponseData` con la respuesta.

use std::time::Instant;

use reqwest::Method;

use crate::app::{PendingRequest, ResponseData, ResponseModel, duration_to_ms, pretty_json_or_raw};

pub async fn send_request(request: PendingRequest) -> ResponseData {
    let client = reqwest::Client::new();
    let method = Method::from_bytes(request.method.as_bytes()).unwrap_or(Method::GET);
    let mut req = client.request(method, request.url);

    for (key, value) in request.headers {
        if !key.trim().is_empty() {
            req = req.header(key, value);
        }
    }

    if !request.body.trim().is_empty() {
        req = req.body(request.body);
    }

    let started = Instant::now();
    match req.send().await {
        Ok(response) => {
            let status = response.status();
            match response.text().await {
                Ok(body) => ResponseData {
                    response: Some(ResponseModel {
                        status: Some(status.as_u16()),
                        status_text: status
                            .canonical_reason()
                            .unwrap_or("Unknown Status")
                            .to_string(),
                        duration_ms: duration_to_ms(started.elapsed()),
                        size_bytes: body.len(),
                        body: pretty_json_or_raw(&body),
                    }),
                    error: None,
                },
                Err(err) => ResponseData {
                    response: None,
                    error: Some(format!("No se pudo leer el body de la respuesta: {err}")),
                },
            }
        }
        Err(err) => ResponseData {
            response: None,
            error: Some(format!("Error enviando request: {err}")),
        },
    }
}
