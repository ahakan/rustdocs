fn main() 
{
    let mut v = vec![1, 2, 3, 4, 5];

    println!("The third element of v is {}", v[2]);

    let i: usize = 0;   // index must be usize

    println!("The first element of v is {}", v[i]);


    let v2 = vec![1, 2, 3];
    match v2.get(2) {
        Some(x) => println!("Item 2 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    // that is error. ownership
    // for i in v {
    //     println!("Take ownership of the vector and its element {}", i);
    // }
    
}
