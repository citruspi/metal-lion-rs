use std::net::SocketAddr;

use rust_embed::RustEmbed;
use warp::{http::HeaderValue, reply::Response, Filter, Rejection, Reply};

#[derive(RustEmbed)]
#[folder = "assets/web"]
struct Asset;

pub async fn serve_file(path: &str, content_type: &str) -> Result<impl Reply, Rejection> {
    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;

    let mut res = Response::new(asset.into());
    res.headers_mut()
        .insert("Content-Type", HeaderValue::from_str(content_type).unwrap());
    Ok(res)
}

pub async fn listen(bind_addr: SocketAddr) {
    let index_html = warp::path::end().and_then(|| serve_file("index.html", "text/html"));

    warp::serve(index_html).run(bind_addr).await
}
