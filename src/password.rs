use rand::prelude::*;

pub fn generate_password(length: u8, alpha: bool) -> String{
    let mut rng = rand::thread_rng();
    let s = String::from("1234567890abcdefghijklmopqrstuvwxyz");
    let mut ans=String::from("");
    let upper = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    for _i in 0..length{
        let mut case=true;
        if alpha {
            case = rng.gen_bool(0.5);
        }
        if case{
            ans.push(s.chars().nth(rng.gen_range(0..s.len())).unwrap());
        }
        else{
            ans.push(upper.chars().nth(rng.gen_range(0..upper.len())).unwrap());
        }
    }
    ans
}