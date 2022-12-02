mod Test;
use pollster;
fn main() {
    println!("Hello, world!");
    pollster::block_on(Test::run());
}
