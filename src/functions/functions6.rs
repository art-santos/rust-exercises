fn main(){
    let get_power = power(2, 100);
    println!("{}", get_power);
}

fn power(base: i128, exponent: i128) -> i128 {
    if exponent == 0 {
        return 1;
    } else {
        return base * power(base, exponent - 1);
    }
}