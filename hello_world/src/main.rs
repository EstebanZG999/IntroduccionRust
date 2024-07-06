mod color;
use color::Color;

fn main() {
    let red = Color::new(255, 0, 0);
    println!("{:?}", red);

    let blue = Color::from_hex(0x0000FF);
    println!("{:?}", blue);

    let hex_value = red.to_hex();
    println!("Hex value: {:X}", hex_value);
}
