mod binary;
use binary::BinaryValue;
mod data_types;
mod init;
use init::FloatInitializer;
mod operations;
use operations as ops;

fn main() {
    let a = BinaryValue::from("0001").unwrap();
    let b = BinaryValue::from("0111").unwrap();
    let c = &a + &b;
    println!("{}", c == BinaryValue::from("1000").unwrap());
    println!("{}", &c - &b == a);
    println!("{}", &c - &a == b);
    println!("{}", -&c == BinaryValue::from("1000").unwrap());
    let (a, b) = (250.0, 0.0025);
    let initializer = FloatInitializer::new(7, 24);
    println!("F1: {} : {:?}", a, initializer.get_f1(a).get());
    println!("F1: {} : {:?}", b, initializer.get_f1(b).get());
    println!("F1: {} : {}", a, initializer.from_f1(initializer.get_f1(a)).unwrap());
    println!("F1: {} : {}", b, initializer.from_f1(initializer.get_f1(b)).unwrap());
    println!("F2: {} : {:?}", a, initializer.get_f2(a).get());
    println!("F2: {} : {:?}", b, initializer.get_f2(b).get());
    println!("F2: {} : {}", a, initializer.from_f2(initializer.get_f2(a)).unwrap());
    println!("F2: {} : {}", b, initializer.from_f2(initializer.get_f2(b)).unwrap());
}
