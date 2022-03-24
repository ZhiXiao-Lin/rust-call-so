#[link(name = "hello")]
extern "C" {
    fn add(a: u32, b: u32);
}

fn main() {
    unsafe {
        add(1, 2);
    }
    println!("Hello, world!");
}
