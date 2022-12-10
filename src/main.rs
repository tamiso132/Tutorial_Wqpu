#![allow(dead_code)]
mod test;

fn main() {
    pollster::block_on(test::run());
}
