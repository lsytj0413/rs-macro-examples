use config_macro::config;

fn main() {
    config!();
    let cfg = Config::new();
    let user = cfg.0.get("user").unwrap();
    println!("{user}");
}
