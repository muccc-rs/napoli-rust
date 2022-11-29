
use napoli_lib::create_example_order;
use prost::Message;

pub fn main() {
    let order = create_example_order();
    println!("order: {:?}", order);
    
    // Serialize the order to a buffer
    let mut buf = vec![0; order.encoded_len()];
    order.encode(&mut buf).unwrap();
    println!("buf: {:?}", buf);

    // Interpret each byte of the buffer as a char and print
    for byte in buf {
        print!("{}", byte as char);
    }
}