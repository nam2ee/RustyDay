fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");

    let r1 = &s;
    let r2 = &s;

    let r3 = r1;

    println!("{}",r1);
    

    let word = first_word(&s); // word는 값 5를 받습니다

    // Slice는 참조자의 일종. 소유권을 가져오지 x
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()
}