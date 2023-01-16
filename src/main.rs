use rust_env::{Env, match_str};

fn main() {
    let mut env = Env::new("./config.env");

    env.raw("PORT=6779\nIP=127;0;0;1");
    env.debug();

    let addr = match_str(env.get_hash("ADDR"));
    println!("{:?}", addr);
}