struct Point
{
    x: i32,     // variables are not define mutable
    y: i32,
}


struct Color(u32, u32, u32); // tuple struct

fn main() 
{
    let origin = Point { x: 0, y: 0 };

    let mut new_point = Point { x: 15, y: 20 };
    
    println!("The point is at ({}, {})", new_point.x, new_point.y);

    new_point.x = 8;

    println!("The new point is at ({}, {})", new_point.x, new_point.y);

    new_point = Point { x: 5, .. origin};

    println!("The new point is at ({}, {})", new_point.x, new_point.y);


    // tuple struct

    let blue = Color(0, 0, 255);

    println!("Blue color is ({}, {}, {})", blue.0, blue.1, blue.2);

}
