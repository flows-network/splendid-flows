use serde_json::Value;
use std::collections::HashMap;
use webhook_flows::{create_endpoint, request_handler, send_response};

const BACK_BUF: &[u8] = include_bytes!("1000.webp") as &[u8];

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler(
    _headers: Vec<(String, String)>,
    _subpath: String,
    qry: HashMap<String, Value>,
    body: Vec<u8>,
) {
    let mut back = image::load_from_memory(BACK_BUF).unwrap();
    let front = image::load_from_memory(&body).unwrap();
    let x = qry
        .get("x")
        .unwrap_or(&Value::from(0))
        .as_i64()
        .unwrap_or(0);
    let y = qry
        .get("y")
        .unwrap_or(&Value::from(0))
        .as_i64()
        .unwrap_or(0);
    image::imageops::overlay(&mut back, &front, x, y);
    let src_buf = back.as_bytes();

    let mut target_buf = std::io::Cursor::new(Vec::new());
    match image::write_buffer_with_format(
        &mut target_buf,
        src_buf,
        back.width(),
        back.height(),
        back.color(),
        image::ImageFormat::Png,
    ) {
        Ok(_) => {
            send_response(
                200,
                vec![
                    (String::from("Content-Type"), String::from("image/png")),
                    (
                        String::from("Access-Control-Allow-Origin"),
                        String::from("*"),
                    ),
                ],
                target_buf.into_inner(),
            );
        }
        Err(e) => {
            send_response(500, vec![], e.to_string().as_bytes().to_vec());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
