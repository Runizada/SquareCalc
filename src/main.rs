use std::io;

fn main() {
    let a: i32 = loop {
        let mut imput1 = String::new();

        println!("Coloque o valor de A \n\n");
        io::stdin().read_line(&mut imput1).expect("erro no imput 1");
        match imput1.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => println!("Isso não é um número"),
        }
    };

    let b: i32 = loop {
        let mut imput2 = String::new();

        println!("Coloque o valor de B \n\n");
        io::stdin().read_line(&mut imput2).expect("erro no imput 2");
        match imput2.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => println!("Isso não é um número"),
        }
    };

    let c: i32 = loop {
        let mut imput3 = String::new();

        println!("Coloque o valor para C \n\n");
        io::stdin().read_line(&mut imput3).expect("erro no imput 3");
        match imput3.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => println!("Isso não é um número"),
        }
    };

    let n1: i32 = 4;
    let n2: i32 = 2;

    let d = b.pow(n2.try_into().expect("falha")) - n1 * a * c;

    if d.is_negative() {
        println!("o Delta é negativo \nA função não tem raizes reais");
        return;
    }

    let dr = (d as f64).sqrt();

    let x1 = (-(b as f64) + dr) / ((n2 as f64) * (a as f64));
    let x2 = (-(b as f64) - dr) / ((n2 as f64) * (a as f64));

    println!("os raizes da função são x'={} x''={}\n\n", x1, x2);
}
