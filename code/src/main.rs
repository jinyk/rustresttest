extern crate curl;

use curl::http;

fn main() {
    let url = "https://www.hautelook.com/api";
    let resp = http::handle()
        .get(url)
        .exec()
        .unwrap_or_else(|e| {
            panic!("Failed to get {}; error is {}", url, e);
        });
}
