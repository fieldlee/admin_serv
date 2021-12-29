#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate fluffy;

mod config;


fn main() {
    println!("Hello, world!");
    println!("{:?}",config::get_conn_string());
}
