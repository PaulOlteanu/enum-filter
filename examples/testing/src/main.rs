use enum_filter::enum_filter;

#[enum_filter]
enum TestEnum {
    V1,
    V2(u32),
    V3 { a: u32 },
}

fn main() {
    println!("Hello, world!");
}
