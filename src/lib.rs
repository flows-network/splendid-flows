use flowsnet_platform_sdk::logger;
use serde_json::Value;
use std::collections::HashMap;
use webhook_flows::{create_endpoint, request_handler, send_response};

const BACK_BUF: &[u8] = include_bytes!("1000.webp") as &[u8];

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

fn get_qry(qry: &HashMap<String, Value>, key: &str, default: i64) -> i64 {
    qry.get(key)
        .unwrap_or(&Value::from(""))
        .as_str()
        .unwrap_or("")
        .parse()
        .unwrap_or(default)
}

#[request_handler]
async fn handler(
    _headers: Vec<(String, String)>,
    _subpath: String,
    qry: HashMap<String, Value>,
    body: Vec<u8>,
) {
    logger::init();
    let mut front = image::load_from_memory(&body).unwrap();
    let w: u32 = get_qry(&qry, "w", front.width() as i64) as u32;
    let h: u32 = get_qry(&qry, "h", front.height() as i64) as u32;

    if front.width() != w || front.height() != h {
        front.resize(w, h, image::imageops::Lanczos3);
    }

    let l: i64 = get_qry(&qry, "l", 0);
    let t: i64 = get_qry(&qry, "t", 0);
    let mut back = image::load_from_memory(BACK_BUF).unwrap();
    image::imageops::overlay(&mut back, &front, l, t);
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
