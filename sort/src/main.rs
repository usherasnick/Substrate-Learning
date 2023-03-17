fn main() {
    let mut vec = [1, 5, 10, 2, 15];
    let mut i = 0;

    while i < vec.len()-1 {
        let mut j = 0;
        while j < vec.len()-1-i {
            if vec[j] > vec[j+1] {
                let tmp = vec[j+1];
                vec[j+1] = vec[j];
                vec[j] = tmp;

            }
            j = j + 1;
        }
        i = i + 1;
    }
    println!("{:?}", vec);
}

