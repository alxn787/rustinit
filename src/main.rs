use std::f32::consts::PI;

enum Shapes {
    Circle(f32),
    RecTangle(f32,f32),
    Square(f32)
}

fn main(){
    let shape_circle = Shapes::Circle(10.0);
    let shape_rect = Shapes::RecTangle(10.0, 15.0);
    let shape_square = Shapes::Square(10.0);

    println!("{}" , area(&shape_circle));
    println!("{}" , area(&shape_rect));
    println!("{}" , area(&shape_square));

}

fn area(shape:&Shapes)->f32{
    match shape {
        Shapes::Circle(r) => r * r * PI,
        Shapes::RecTangle(l,b ) => l * b,
        Shapes::Square(s) => s * s
        
    }
}