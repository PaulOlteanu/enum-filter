use std::iter::FilterMap;

use enum_filter::enum_filter;

#[enum_filter]
enum TestEnum {
    V1,
    V2(u32),
    V3 { a: u32 },
}

fn main() {
    let mut temp = [TestEnum::V1, TestEnum::V2(2), TestEnum::V3 { a: 3 }];
    let result: Vec<_> = temp.iter().filter_v2().collect();
    println!("{:?}", result);
    let result: Vec<_> = temp.iter_mut().filter_v2().collect();
    println!("{:?}", result);
    let result: Vec<_> = temp.into_iter().filter_v2().collect();
    println!("{:?}", result);
}
