use std::io;

fn f() {
    println!("Type your value in °F:");

    let f: f32 = {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Não foi possível acessar o stdin");
        buffer.trim().parse().expect("Por favor, apenas números!")
    };

    let result: f32 = (f - 32.0) * 5.0/9.0;

    println!("{}°F = {}°C", f, result);
}

fn c() {
    println!("Type your value in °C:");

    let c: f32 = {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Não foi possível acessar o stdin");
        buffer.trim().parse().expect("Por favor, apenas números!")
    };

    let result: f32 = (c * 9.0/5.0) + 32.0;

    println!("{}°C = {}°F", c, result);
}

fn main() {

    println!("Would you like to convert from (1)°F to °C or (2)°C to °F");

    let scale: u8 = {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Não foi possível acessar o stdin");
        buffer.trim().parse().expect("Por favor, apenas números!")
    };
    
    if scale == 1 {
        f();
    } else {
        c();
    }

}