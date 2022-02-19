fn main(){
    call_me(10000);
}

fn call_me(x: i64){
    for i in 0..x{
        println!("Ring! Call number {}", i);
    }
}