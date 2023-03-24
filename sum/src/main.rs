fn main() {
    let a = [10,20,30,u32::MAX-1
    ];
    match sum(&a[..]) {
        Some(value) => {
            println!("sum value: {}", value)
        },
        None => {
            println!("value overflow");
        },
    }
}

fn sum(input: &[u32]) -> Option<u32> {
    let mut sum:u32 = 0;
    for i in input {
        match sum.checked_add(*i) {
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
