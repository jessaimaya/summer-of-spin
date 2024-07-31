use serde::{Deserialize, Serialize};
use serde_json::json;
use spin_sdk::http::{send, IntoResponse, Method, Request, Response};
use spin_sdk::http_component;

#[derive(Debug, Serialize, Deserialize)]
struct CryptoResponse {
    #[serde(rename(deserialize = "requestBody"))]
    request_body: String,
    #[serde(rename(deserialize = "actionType"))]
    action_type: String,
    response: String,
}
/// A simple Spin HTTP component.
#[http_component]
async fn handle_challenge1(req: Request) -> anyhow::Result<impl IntoResponse> {
    // println!("Handling request to {:?}", req.header("spin-full-url"));
    match req.method() {
        Method::Post => {
            if req.path().contains("crypto") {
                return Ok(Response::builder().status(200).body("").build());
            }
            let body: Vec<u8> = req.body().into();
            let crypto_url = String::from(format!("{}crypto", req.uri()));
            let body_str = String::from_utf8_lossy(&body).to_string();
            let decrypt_req = Request::builder()
                .uri(crypto_url.clone())
                .method(Method::Post)
                .header("x-action", "decrypt")
                .body(body_str)
                .build();
            let res: Response = send(decrypt_req).await?;
            let data = res.body().to_owned();
            let data_str = String::from_utf8_lossy(&data).to_string();
            let data_json: CryptoResponse =
                serde_json::from_str(&data_str).expect("unable to parse into CryptoResponse");
            let x_secret_play = "jess";
            let new_data = String::from(format!("{}{}", data_json.response, x_secret_play));

            let encrypt_req = Request::builder()
                .uri(crypto_url.clone())
                .method(Method::Post)
                .header("x-action", "encrypt")
                .body(new_data)
                .build();
            let encrypt_res: Response = send(encrypt_req).await?;
            let encrypt_res_body = encrypt_res.body();
            let encrypt_body_str: String = String::from_utf8_lossy(encrypt_res_body).to_string();
            let data_json: CryptoResponse = serde_json::from_str(&encrypt_body_str)
                .expect("unable to parse into CryptoResponse for encrypt");

            let body_json = json!({
                "encryptedMessage": data_json.response
            });

            return Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("x-secret-play", x_secret_play)
                .header("x-encryption-module-path", "crypto")
                .body(body_json.to_string())
                .build());
        }
        _ => {
            return Ok(Response::builder()
                .status(200)
                .header("content-type", "text/plain")
                .body("Hello, Fermyon")
                .build())
        }
    };
}
