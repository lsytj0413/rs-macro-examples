use config_macro::{config, config_struct};


#[config_struct(path = "./configuration/config.yaml", )]
#[derive(Debug)]
struct ConfigObject {

}

fn main() {
    config!();
    let cfg = Config::new();
    let user = cfg.0.get("user").unwrap();
    println!("{user}");

    let c = ConfigObject::new();
    println!("{:?}", c);
}
