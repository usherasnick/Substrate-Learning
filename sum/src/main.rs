fn main() {
    let a:[u8;4] = [10,20,30,25];
    let res = sum(&a[..]);
    match res {
        Some(value) => {
            println!("sum value {}", value)
        },
        None => {
            println!("value overflow");
        },
    }
}

fn sum(input: &[u8]) -> Option<u8> {
    let mut sum:u8 = 0;
    for i in input.iter() {
        let res = sum.checked_add(*i);
        match res {
            Some(value) => {
                sum = value;
                continue
            },
            None => {
                return None;
            }
        }
    }
    Some(sum)
}
