fn main() {
    let mut a: u32;
    a = 112;
    println!("Число 1: {}", a);
    a = 12;
    let name= "Иван";
    println!("Число {} 2: {}", name, a);
    let name= "Голдшип"; 
    println!("Второе имя {}", name);

    let chislo: i32 = 12;
    println!("{}", chislo);

    println!("{}", apple());
    binary_help();
}


fn apple() -> i32
{
    let mut a = 21;
    a <<= 2;
    if a < 12
    {
        a
    }
    else {
        3
    }
}

fn binary_help()
{
    let a1: i8 = 12;
    println!("{} = {:b}", a1, a1);
    let a2: i8 = 23;
    println!("{} = {:b}", a2, a2);
    let mut result = a1 & a2;
    println!("Конъюнкция {} = {:b}", result, result);
    result = a1 | a2;
    println!("Дизъюнкция {} = {:b}", result, result);
    result = a1 ^ a2;
    println!("Логическое ИЛИ {} = {:b}", result, result);
}