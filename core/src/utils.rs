#[cfg(feature = "file-body")]
use tokio::fs::File;
#[cfg(feature = "file-body")]
use tokio_util::codec::{BytesCodec, FramedRead};

#[cfg(feature = "file-body")]
pub fn file_to_body(file: File) -> reqwest::Body {
  let stream = FramedRead::new(file, BytesCodec::new());

  reqwest::Body::wrap_stream(stream)
}
