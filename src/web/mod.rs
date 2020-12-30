use lazy_static::lazy_static;
use prometheus::{IntCounter, Registry};
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use warp::{
    http::header::CONTENT_TYPE, http::HeaderValue, reply::Response, Filter, Rejection, Reply,
};

use crate::badges;

lazy_static! {
    pub static ref PROM_REGISTRY: Registry = Registry::new();
    pub static ref PROM_BADGE_RENDERS: IntCounter =
        IntCounter::new("badge_renders", "Badges rendered").expect("metric can be created");
    pub static ref PROM_BADGE_RENDER_ERRORS: IntCounter =
        IntCounter::new("badge_render_errros", "Failed badge renders")
            .expect("metric can be created");
}

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
            "render_endpoint": f.render_endpoint(),
            "version": crate_version!(),
        }))
        .unwrap()
}

pub async fn render_svg_badge(
    factory: badges::Factory,
    input: badges::SvgBadgeInput,
) -> Result<impl Reply, Rejection> {
    let mut res: Response;

    PROM_BADGE_RENDERS.inc();

    match factory.render_svg(input) {
        Ok(badge) => {
            res = Response::new(badge.into());

            res.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("image/svg+xml;charset=utf-8"),
            );
        }
        Err(err) => {
            PROM_BADGE_RENDER_ERRORS.inc();

            res = Response::new(factory.render_error_badge(err).into());

            res.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("image/svg+xml;charset=utf-8"),
            );
        }
    };

    Ok(res)
}

async fn prometheus_metrics_handler() -> Result<impl Reply, Rejection> {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&PROM_REGISTRY.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    };
    let res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    Ok(res)
}

pub async fn listen(bind_addr: SocketAddr, factory: badges::Factory) {
    PROM_REGISTRY
        .register(Box::new(PROM_BADGE_RENDERS.clone()))
        .expect("collector can be registered");

    PROM_REGISTRY
        .register(Box::new(PROM_BADGE_RENDER_ERRORS.clone()))
        .expect("collector can be registered");

    let index_html = render_index(factory.clone());

    let index_page = warp::path::end().map(move || warp::reply::html(index_html.clone()));

    let svg_badge = warp::path!("v1" / "badge.svg")
        .and(warp::query::<badges::SvgBadgeInput>())
        .and_then(move |input| render_svg_badge(factory.clone(), input));

    let metrics_route = warp::path!("metrics").and_then(prometheus_metrics_handler);

    warp::serve(index_page.or(svg_badge).or(metrics_route))
        .run(bind_addr)
        .await
}
