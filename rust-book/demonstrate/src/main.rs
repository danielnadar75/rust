use std::io;

fn main() {
    println!("*** Welcome to C-F converter ***\n");

    println!("Options>");
    println!("1. C -> F Conversion");
    println!("2. F -> C Conversion\n");

    println!("Enter your choice: ");
    let mut option: String = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Error reading from stdin");

    let option: u8 = option.trim().parse().expect("Not a number!");

    if option == 1 {
        println!("Converting C -> F");

        println!("Enter Degree In Celcius: ");
        let mut celcuis = String::new();

        io::stdin()
            .read_line(&mut celcuis)
            .expect("Error reading celcius from stdion");

        let celcius: f32 = celcuis.trim().parse().expect("Celcius is not a number");

        let feh = celcius * (9.0 / 5.0) + 32.0;

        println!("Temprature in F is {feh}");
    } else if option == 2 {
        println!("Converting F -> C");

        println!("Enter Degree In Fahrenheit : ");

        let mut feh = String::new();

        io::stdin()
            .read_line(&mut feh)
            .expect("Error reading Fahrenheit from the stdin");

        let feh: f32 = feh.trim().parse().expect("Fehrenheit is not a number!");

        let celcius: f32 = feh - 32.0 * (9.0 / 5.0);

        println!("Temprature in c is {celcius}");

    } else {
        println!("Invalid Option! Try again later!")
    }
}
