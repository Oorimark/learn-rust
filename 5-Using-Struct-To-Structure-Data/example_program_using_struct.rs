/*
*  Calculate the area of a rectangle
*/

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rect) -> bool {
        rect.width <= self.width && rect.height <= self.height
    }
}

fn main() {
    let sq = Rect::square(3);
    let rect_1 = Rect {
        width: dbg!(30),
        height: 50,
    };
    let rect_2 = Rect {
        width: dbg!(10),
        height: 40,
    };
    let rect_3 = Rect {
        width: dbg!(60),
        height: 45,
    };
    let area = rect_1.area();

    dbg!(&sq);
    println!(
        "The width = {},\nheight = {},\nArea = {}",
        rect_1.width, rect_2.height, area,
    );
    println!("Can react 1 hold react 2 {}", rect_1.can_hold(&rect_2));
    println!("Can react 1 hold react 3 {}", rect_1.can_hold(&rect_3));
}

// fn area_rect(rect: &Rect) -> u32 {
//     rect.width * rect.height
// }
