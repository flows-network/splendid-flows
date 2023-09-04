use serde_json::Value;
use std::collections::HashMap;
use webhook_flows::{create_endpoint, request_handler, send_response};

const BACK_BUF: &[u8] = include_bytes!("1000.webp") as &[u8];
const FRONT_BUF: &[u8] = include_bytes!("NASA.png") as &[u8];

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler(
    _headers: Vec<(String, String)>,
    _subpath: String,
    _qry: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    let mut back = image::load_from_memory(BACK_BUF).unwrap();
    let front = image::load_from_memory(FRONT_BUF).unwrap();
    image::imageops::overlay(&mut back, &front, 0, 0);
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
                vec![(String::from("content-type"), String::from("image/png"))],
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
