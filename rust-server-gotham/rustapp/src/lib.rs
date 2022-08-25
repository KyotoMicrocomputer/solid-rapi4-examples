#![deny(unsafe_op_in_unsafe_fn)]
use hyper::StatusCode;

#[cfg(target_os = "solid_asp3")]
mod cpumon;
#[cfg(target_os = "solid_asp3")]
mod stubs;

/// The root task entry point
#[no_mangle]
extern "C" fn slo_main() {
    // Register a logger
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .init();

    // Start CPU usage monitor
    #[cfg(target_os = "solid_asp3")]
    cpumon::init();

    // Start Rayon worker threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(solid::abi::SOLID_CORE_MAX)
        .start_handler(|i| {
            #[cfg(target_os = "solid_asp3")]
            {
                let task = itron::task::current().unwrap();
                let task = task.as_ref();

                task.migrate(itron::processor::Processor::from_raw(i as i32 + 1).unwrap())
                    .unwrap();

                // Lower the task priority
                task.set_priority(task.priority().unwrap() + 1).unwrap();
            }
        })
        .build_global()
        .unwrap();

    // Initialize Tokio
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(solid::abi::SOLID_CORE_MAX)
        .thread_name("tokio worker")
        .on_thread_start(|| {
            #[cfg(target_os = "solid_asp3")]
            {
                // Distribute the worker threads across all processors
                use std::sync::atomic::{AtomicUsize, Ordering};
                static TID: AtomicUsize = AtomicUsize::new(0);
                let thread_index = TID.fetch_add(1, Ordering::Relaxed);
                let i = thread_index % solid::abi::SOLID_CORE_MAX;
                itron::task::current()
                    .unwrap()
                    .as_ref()
                    .migrate(itron::processor::Processor::from_raw(i as i32 + 1).unwrap())
                    .unwrap();
            }
        })
        .max_blocking_threads(20)
        .enable_all()
        .build()
        .unwrap();

    // Start HTTP server
    rt.block_on(server_loop());

    unreachable!();
}

// ----------------------------------------------------------------------------
//                                HTTP Server
// ----------------------------------------------------------------------------

/// Start an HTTP server on the current async task. This functil will never return.
async fn server_loop() -> ! {
    // TODO: Gotham doesn't limit the maximum number of concurrent connections,
    //       which results in gotham-rs/gotham#282
    println!("Listening at 0.0.0.0:8080");
    gotham::plain::init_server("0.0.0.0:8080", router())
        .await
        .unwrap();
    panic!("The HTTP server returned the control unexpectedly")
}

fn router() -> gotham::router::Router {
    use gotham::router::builder::*;
    build_simple_router(|route| {
        // `/`: Index page
        route.get("/").to(static_handler(
            &mime::TEXT_HTML,
            include_bytes!("../static/index.html"),
        ));

        // `/v0/panic`: Cause a panic
        route
            .post("/v0/panic")
            .to(|_| -> (State, &'static str) { panic!("panic requested by the user") });

        // `/v0/sensors`: Get latest measurements
        route.get("/v0/sensors").to(handle_sensors);

        // `/v0/mbs/:x/:y/:r`: Visualize a mandelbrot set
        for pattern in ["/v0/mbs", "/v0/mbs/:x", "/v0/mbs/:x/:y", "/v0/mbs/:x/:y/:r"] {
            route
                .get(pattern)
                .with_path_extractor::<MbsParams>()
                .to(handle_mbs);
        }

        // `/v0/fetch/:url`: Generate an HTTP request and return the response
        route
            .get("/v0/fetch/*")
            .with_path_extractor::<FetchParams>()
            .to(handle_fetch);
    })
}

// ----------------------------------------------------------------------------
//                              Request Handlers
// ----------------------------------------------------------------------------

use gotham::{
    handler::IntoResponse,
    helpers::http::response::create_response,
    state::{FromState, State},
};
use gotham_derive::{StateData, StaticResponseExtender};
use http::HeaderValue;
use serde::Deserialize;

fn static_handler(
    mime: &'static mime::Mime,
    body: &'static [u8],
) -> impl gotham::handler::Handler + Copy + Send + Sync {
    move |st| (st, (mime.clone(), body))
}

/// `/v0/sensors`
fn handle_sensors(st: State) -> (State, impl IntoResponse) {
    #[cfg(target_os = "solid_asp3")]
    let [cpu0] = cpumon::current_cpu_usage();
    #[cfg(not(target_os = "solid_asp3"))]
    let [cpu0] = [0.5f32];

    let body = format!(r#"{{"cpu":[{cpu0}]}}"#);
    (st, (mime::APPLICATION_JSON, body))
}

/// Parameteres for `/v0/mbs`
#[derive(Deserialize, StateData, StaticResponseExtender)]
#[serde(default)]
struct MbsParams {
    x: f32,
    y: f32,
    r: f32,
}

impl Default for MbsParams {
    fn default() -> Self {
        Self {
            x: -0.725,
            y: 0.2,
            r: 0.005,
        }
    }
}

/// `/v0/mbs`
fn handle_mbs(mut st: State) -> (State, impl IntoResponse) {
    let MbsParams { x, y, r } = MbsParams::take_from(&mut st);
    let (image, elapsed) = render_mandelbrot_set(x, y, r);
    let mut response = create_response(&st, StatusCode::OK, mime::IMAGE_JPEG, image);
    response.headers_mut().insert(
        "X-Render-Time",
        HeaderValue::from_str(&format!("{}s", elapsed)).unwrap(),
    );
    (st, response)
}

/// Parameters for `/v0/fetch/:url`
#[derive(Deserialize, StateData, StaticResponseExtender)]
struct FetchParams {
    /// `http://aaa/bbb` → `["http:", "", "aaa", "bbb"]`
    #[serde(rename = "*")]
    proxied_uri_segs: Vec<String>,
}

/// `/v0/fetch/:url`
fn handle_fetch(mut st: State) -> (State, impl IntoResponse) {
    let FetchParams {
        mut proxied_uri_segs,
    } = FetchParams::take_from(&mut st);
    if proxied_uri_segs.len() >= 2
        && proxied_uri_segs[0].ends_with(":")
        && !proxied_uri_segs[1].is_empty()
    {
        // Bring back the dissolved empty section between two slashes in `http://`
        proxied_uri_segs.insert(1, String::new());
    }
    let proxied_uri = proxied_uri_segs.join("/");

    let mut writer = Vec::new();
    let response = match http_req::request::get(&proxied_uri, &mut writer) {
        Ok(_) => create_response(&st, StatusCode::OK, mime::TEXT_PLAIN, writer),
        Err(e) => gotham::helpers::http::response::create_response(
            &st,
            StatusCode::BAD_GATEWAY,
            mime::TEXT_PLAIN,
            format!("HTTP request to {:?} failed: {:?}", proxied_uri, e),
        ),
    };

    (st, response)
}

fn render_mandelbrot_set(vp_x: f32, vp_y: f32, vp_r: f32) -> (Vec<u8>, f64) {
    const IMGDIM: usize = 512;
    const THRESHOLD: f32 = 100.0;

    use image::ImageEncoder;
    use rayon::prelude::*;

    type F32xN = packed_simd::f32x4;

    let mut imgbuf = image::RgbaImage::new(IMGDIM as u32, IMGDIM as u32);
    let start = std::time::Instant::now();

    // Compute in Rayon worker threads
    let mut rows: Vec<_> = imgbuf.rows_mut().collect();
    rows.par_iter_mut().enumerate().for_each(|(pix_y, row)| {
        let pixel_size_x = vp_r * (2.0 / IMGDIM as f32);
        let pixel_size_y = pixel_size_x;

        // 4x4 uniform sampling
        let msaa_pattern_x = F32xN::new(-1.5, -0.5, 0.5, 1.5) / 4.0 * pixel_size_x;
        let msaa_pattern: [[F32xN; 2]; 4] = [
            [msaa_pattern_x, F32xN::splat(-1.5 / 4.0) * pixel_size_y],
            [msaa_pattern_x, F32xN::splat(-0.5 / 4.0) * pixel_size_y],
            [msaa_pattern_x, F32xN::splat(0.5 / 4.0) * pixel_size_y],
            [msaa_pattern_x, F32xN::splat(1.5 / 4.0) * pixel_size_y],
        ];

        let pix_y = pix_y as f32 * pixel_size_y + (vp_y - vp_r);

        for (pix_x, pixel) in row.enumerate() {
            let pix_x = pix_x as f32 * pixel_size_x + (vp_x - vp_r);

            let coverage: u32 = msaa_pattern
                .iter()
                .map(|msaa_sample| {
                    let pix_x = F32xN::splat(pix_x) + msaa_sample[0];
                    let pix_y = F32xN::splat(pix_y) + msaa_sample[1];

                    let c = [pix_x, pix_y];
                    let mut z = c;
                    let mut divergence_neg = Default::default();

                    for _ in 0..256 {
                        // z = z² + c
                        z = [
                            // z[0]*z[0] - z[1]*z[1] + c[0]
                            (-z[1]).mul_adde(z[1], z[0] * z[0]) + c[0],
                            // 2*z[0]*z[1] + c[1]
                            (z[0] * z[1]).mul_adde(F32xN::splat(2.0), c[1]),
                        ];

                        // divergence = |z|² ≥ THRESHOLD
                        divergence_neg =
                            F32xN::splat(THRESHOLD).gt(z[0].mul_add(z[0], z[1] * z[1]));

                        // if all_simd_lanes(divergence) { break; }
                        if !divergence_neg.any() {
                            break;
                        }
                    }

                    divergence_neg.bitmask().count_ones()
                })
                .sum();

            let coverage = coverage as f32 / (F32xN::lanes() * msaa_pattern.len()) as f32;
            let luma = ((1.0 - coverage).sqrt() * 255.0) as u8;
            let color = image::Rgba([luma, luma, luma, 255]);

            *pixel = color;
        }
    });

    let elapsed = start.elapsed().as_secs_f64();

    // Encode as a JPEG image
    let mut encoded = Vec::with_capacity(IMGDIM * IMGDIM / 4);
    let j = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut encoded, 90);
    j.write_image(&imgbuf, IMGDIM as _, IMGDIM as _, image::ColorType::Bgra8)
        .unwrap();
    (encoded, elapsed)
}
