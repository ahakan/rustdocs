fn main() 
{
    let mut i = 0;


    // LOOP
    loop
    {
        i += 1;

        println!("Loop forever!");

        if i == 3
        {
            break;
        }
    }

    let mut x = 0;

    let mut done = false;


    // WHILE LOOP
    while !done
    {
        x += 1;

        if x == 5
        {
            println!("x is {}!", x);

            done = true;
        }
    }


    // FOR LOOPS
    for x in 0..10
    {
        println!("x is {}!", x);
    }


    for (index, value) in (5..10).enumerate()
    {
        println!("index = {} and value = {}.", index, value);
    }


    // Loop labels
    'outer: for x in 0..10
    {
        'inner: for y in 0..10
        {
            if x % 2 == 0 { continue 'outer ;}

            if y % 5 == 0 { continue 'inner; }

            println!("x: {} - y: {}", x, y);
        }
    }
}
