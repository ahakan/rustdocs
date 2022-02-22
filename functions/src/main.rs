fn main() 
{
    print_sum(40, 50);

    let total: u64 = sum (50, 50);

    println!("returned sum: {}", total);

    let f: fn(u64, u64) -> u64 = sum;       // without type interface

    println!("without type interface sum: {}", f(10, 11));

    let k = sum;                            // with type interface

    println!("with type interface sum: {}", k(5, 8));

}

fn print_sum(x: u64, y: u64)
{
    println!("sum: {}", x + y);
}


fn sum(x: u64, y: u64) -> u64
{
    x+y
}