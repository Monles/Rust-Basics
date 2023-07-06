enum Flavour {
    Sweet,
    Sparkling,
    Fruity,
}

struct Drink {
    flavour: Flavour,
    fluid_oz: f64,
}

fn print_drinks(the_drink: Drink) {
    match the_drink.flavour {
        Flavour::Sweet => println!("Sweet Drink!"),
        Flavour::Sparkling => println!("Sparkling Drink!"),
        Flavour::Fruity => println!("Fruity Drink!"),
    }
    println!("Oz: {:?}", the_drink.fluid_oz);
}

fn main() {
    let a: Drink = Drink {
        flavour: Flavour::Sparkling,
        fluid_oz: 10.5,
    };
    print_drinks(a);
}
