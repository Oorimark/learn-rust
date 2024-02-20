/*
*  Calculate the area of a rectangle
*/

struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rect {
        width: 100,
        height: 100,
    };
    let area = area_rect(&rect);

    println!(
        "The width = {},\nheight = {},\nArea = {}",
        rect.width, rect.height, area,
    )
}

fn area_rect(rect: &Rect) -> u32 {
    rect.width * rect.height
}
