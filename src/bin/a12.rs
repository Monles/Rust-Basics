enum Colour {
    Brown,
    Red,
}

impl Colour {
    fn print(&self) {
        match self {
            Colour::Brown => println!("Brown"),
            Colour::Red => println!("Red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    colour: Colour,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, colour: Colour, dimensions: Dimensions) -> Self {
        Self {
            colour,
            weight,
            dimensions,
        }
    }

    fn print(&self) {
        self.colour.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight)
    }
}

fn main() {
    let small_dimensions: Dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let small_box: ShippingBox = ShippingBox::new(5.0, Colour::Red, small_dimensions);
    small_box.print();
}
