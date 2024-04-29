#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("\u{26EC} Red"),
            Color::Green => println!("\u{26EC} Green"),
            Color::Blue => println!("\u{26EC} Blue"),
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
        println!("\u{26EC} width: {:?}", self.width);
        println!("\u{26EC} height: {:?}", self.height);
        println!("\u{26EC} depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    weight: f64,
    color: Color,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("\u{26EC} weight: {:?}", self.weight);
    }
}

struct Temperature {
    degree: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degree: 32.5 }
    }

    fn boiling() -> Self {
        Self { degree: 100.1 }
    }

    fn show_temp(&self) {
        println!("\u{26EC} {:?} degree", self.degree);
    }
}

pub fn struct_run() {
    // ShippingBox
    let small_dimensions = Dimensions {
        width: 1.2,
        height: 2.75,
        depth: 3.9,
    };
    let small_box = ShippingBox::new(5.3, Color::Red, small_dimensions);
    small_box.print();

    // Temperature
    let hot = Temperature { degree: 99.9 };
    hot.show_temp();
    let cold = Temperature::freezing();
    cold.show_temp();
    let boiling = Temperature::boiling();
    boiling.show_temp();
}
