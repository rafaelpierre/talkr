use warp::Filter;

#[tokio::main]
async fn main() {
    // Serve static files from the `web` directory under the `/static` path
    let static_files = warp::path("static").and(warp::fs::dir("web/"));

    // Serve `index.html` at the root path
    let root = warp::path::end().and(warp::fs::file("web/index.html"));

    // Combine the filters
    let routes = root.or(static_files);

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
