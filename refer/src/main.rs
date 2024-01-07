fn main() {
    let mut m = Vec::new();
    m.push("one".to_string());
    m.push("two".to_string());
    show(&m);

    let x = 10;
    let r = &x;
    //assert!(r == 10); // 그냥 쌩 값을 참조할 때는 레퍼런스로 참조된다.
    assert!(*r == 10); 
    println!("{}", r.to_string()); // 그런데, 메소드를 호출하는 . 연산자는 알아서 역참조를 수행한다.

    let mut v = vec!["22","1321","112"];
    //그리고 사실 따로 레퍼런스를 선언을 하지 않았더라도,
    //메소드를 호출했으면, 암묵적으로 빌려온다.
    v.sort();
}
fn show(vec:&Vec<String>){
    for v in vec{
        println!("{}",v);
    }
}