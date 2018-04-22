struct Color{
    red : u8,
    green : u8,
    blue : u8,
}

fn main(){
    let mut x = 10;
    {
        let v = &mut x;

        println!("{}",v);
    }
    x = 9;
    
    println!("{}",x);

    let c = Color{red:0,green:128,blue:33};

    println!("{},{},{}",c.red,c.green,c.blue);

    let numbers = [1,2,3,4,5];

    println!("{}",numbers[0]);

    for n in numbers.iter(){
        println!("{}",n);
    }
}
