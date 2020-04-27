use std::net::SocketAddr;

use rust_embed::RustEmbed;
use warp::{
    http::header::CONTENT_TYPE, http::HeaderValue, reply::Response, Filter, Rejection, Reply,
};

use crate::badges;

#[derive(RustEmbed)]
#[folder = "assets/web"]
struct Asset;

pub fn render_index(f: badges::Factory) -> String {
    let template: String = std::str::from_utf8(Asset::get("index.html").unwrap().as_ref())
        .unwrap()
        .into();

    liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&template)
        .unwrap()
        .render(&liquid::object!({
            "font_faces": format!("[{}]", f.font_faces().join(", ")),
            "font_sizes": format!("[{}]", f.font_sizes().join(", ")),
            "default_font_face": f.default_font_face(),
            "default_font_size": f.default_font_size(),
        }))
        .unwrap()
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
    let index_html = render_index(factory.clone());

    let index_page = warp::path::end().map(move || warp::reply::html(index_html.clone()));

    let svg_badge = warp::path!("v1" / "badge.svg")
        .and(warp::query::<badges::SvgBadgeInput>())
        .and_then(move |input| render_svg_badge(factory.clone(), input));

    warp::serve(index_page.or(svg_badge)).run(bind_addr).await
}
