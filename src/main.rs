struct Rect {
    width:u32,
    height:u32
}

impl Rect {
    fn area (&self)->u32{
        return self.width * self.height;
    }
}

fn main() {
    let r = Rect{
        width:10,
        height:15
    };
    print!("{} {}", r.width, r.area())
}
