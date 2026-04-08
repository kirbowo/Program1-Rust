use std::io::stdin;
fn main() {
    let mut s=String::new();
    let mut arm:bool;
    let mut r:i32;
    let mut n2:i32=0;    
    stdin().read_line(&mut s);
    let mut n: i32 = s
        .trim() 
        .parse()
        .expect("error");
    let l = n.to_string().replace("-","").len();
        for i in 1..l{ 
    r = n % 10;
    n2=(r*r*r)+n2;
    if n2 == n{
        arm=true;
    }else{
        arm=false;
    }
    println!("{}",arm) }
     
}
