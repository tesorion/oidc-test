use openidconnect::IssuerUrl;
use openidconnect::core::{CoreProviderMetadata};
use openidconnect::reqwest::http_client;

fn main() {
    let issuer_url = std::env::args().nth(1).unwrap_or_else(|| panic!("Usage: cargo run <issuer-url>"));
    let issuer_url = IssuerUrl::new(issuer_url).unwrap();
    let md = CoreProviderMetadata::discover(&issuer_url, http_client).unwrap();
    println!("{:?}", md);
}
