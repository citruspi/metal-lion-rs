use std::net::SocketAddr;

use rust_embed::RustEmbed;
use warp::{
    http::header::CONTENT_TYPE, http::HeaderValue, reply::Response, Filter, Rejection, Reply,
};

use crate::badges;

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

pub async fn render_svg_badge(
    factory: badges::Factory,
    input: badges::SvgBadgeInput,
) -> Result<impl Reply, Rejection> {
    let mut res: Response;

    match factory.render_svg(input) {
        Ok(badge) => {
            res = Response::new(badge.into());

            res.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("image/svg+xml;charset=utf-8"),
            );
        }
        Err(err) => res = Response::new(err.into()),
    };

    Ok(res)
}

pub async fn listen(bind_addr: SocketAddr, factory: badges::Factory) {
    let index_html = warp::path::end().and_then(|| serve_file("index.html", "text/html"));

    let svg_badge = warp::path!("v1" / "badge.svg")
        .and(warp::query::<badges::SvgBadgeInput>())
        .and_then(move |input| render_svg_badge(factory.clone(), input));

    warp::serve(index_html.or(svg_badge)).run(bind_addr).await
}
