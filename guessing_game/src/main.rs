use std::io;
use rand::Rng; //trait
use std::cmp::Ordering;

fn main() {
    println!("TEBAK ANGKA!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Tolong masukan angka.");

        // Variable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Angka kamu adalah: {}", guess);


        match guess.cmp(&secret_number){ //ARM = lengang
            Ordering::Less => println!("Terlalu Kecil!"),
            Ordering::Greater => println!("Terlalu Besar!"),
            Ordering::Equal => {
                println!("Kamu Menang!");
                break;
            }
        };
    }

}
