#[cfg(all(feature = "tokio", feature = "std"))]
compile_error!("Feature `tokio` and `std` can't be used at the same time!");
#[cfg(all(feature = "tokio-uring", feature = "std"))]
compile_error!("Feature `tokio-uring` and `std` can't be used at the same time!");
#[cfg(all(feature = "tokio", feature = "tokio-uring"))]
compile_error!("Feature `tokio` and `tokio-uring` can't be used at the same time!");

use mime_guess::Mime;
use once_cell::unsync::Lazy;
use std::{
    env,
    path::{Path, PathBuf},
};

const STATIC_DIRECOTRY: Lazy<PathBuf> = Lazy::new(|| {
    env::current_dir()
        .unwrap()
        .join("static")
        .canonicalize()
        .unwrap()
});

#[cfg(not(feature = "std"))]
pub async fn fetch_file(path: impl AsRef<Path>) -> Option<(String, Mime)> {
    let path = STATIC_DIRECOTRY.join(path);

    #[cfg(feature = "tokio")]
    let contents = {
        use tokio::io::AsyncReadExt;

        let mut file = tokio::fs::File::open(&path).await.ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.ok()?;

        contents
    };
    #[cfg(feature = "tokio-uring")]
    let contents = {
        let buf = vec![0u8; 600 * 1024];
        let file = tokio_uring::fs::File::open(&path).await.ok()?;
        let (res, buf) = file.read_at(buf, 0).await;
        let n = res.ok()?;

        String::from_utf8_lossy(&buf[..n]).to_string()
    };
    let mime: Mime = mime_guess::from_path(path).first_or_text_plain();

    Some((contents, mime))
}

#[cfg(feature = "std")]
pub fn fetch_file(path: impl AsRef<Path>) -> Option<(String, Mime)> {
    let path = STATIC_DIRECOTRY.join(path);

    use std::io::prelude::Read;

    let mut file = std::fs::File::open(&path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    let mime: Mime = mime_guess::from_path(path).first_or_text_plain();

    Some((contents, mime))
}
