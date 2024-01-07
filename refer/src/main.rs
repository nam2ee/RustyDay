fn main() {
    let mut m = Vec::new();
    m.push("one".to_string());
    m.push("two".to_string());
    show(&m);

    let x = 10;
    let r = &x;
    let k = r;
    //assert!(r == 10); // 그냥 쌩 값을 참조할 때는 레퍼런스로 참조된다.
    assert!(*r == 10); 
    println!("{}", r.to_string()); // 그런데, 메소드를 호출하는 . 연산자는 알아서 역참조를 수행한다.

    let mut v = vec!["22","1321","112"];
    //그리고 사실 따로 레퍼런스를 선언을 하지 않았더라도,
    //메소드를 호출했으면, 암묵적으로 빌려온다.
    v.sort();
    
    let x = &facto(3); // Rust는 표현식이면 그냥 참조자를 붙여서 결과를 참조할 수 있다
    println!("{}",x);
    //assert!(x == 6);
    assert!(*x == 6);
    /*
    그런데.. 표현식을 참조한다면, 그 표현식의 Ownership을 가지고 있는 변수는 어디간건가?
    Rust에서는 이런 식으로 표현식을 바로 참조해버릴 경우에, 익명 변수를 만들고,
    "그 익명 변수에 대한 참조"를 수행한다!
    
    그런데, 그 익명 변수의 라이프 사이클은 참조를 포용하면서도, 
    최적화되어야하므로 다음과 같이 동작한다
    1.
    let x = &facto(7);처럼 바로 할당 되었을 때
    -> 해당 익명 변수는 x와 생명주기가 완전히 일치한다.
    참고로, ref 변수는 copy type이다.
    2.
    만일 그게 아니라, 연산에 쓰이는 등의 작업이라면,
    해당 실행문이 끝나고 바로 사라지게 된다
     */
    let t = ("tables",123);
    let box_t = Box::new(t);
    let mut lazy_creator : [u32; 6] = [8,9,5,4,5,6];
    lazy_creator.sort();
    shows(&lazy_creator);
    
    let buffer = [0i32;231];

    let mut chaos = [3,5,4,1,2];
    chaos.sort();
    /*
    내부 동작은 다음과 같다.
    사실, len() 혹은 sort() 같은 것들은 전부
    slice type이지, array type이 내장하고 있는 메소드는 아니다.
    따라서, . 연산자가 호출되는 시점에는
    다른 ownership variable들이 . 호출할 때 
    암묵적으로 레퍼런스를 가져오는 것 처럼,
    암묵적으로 slice reference인 &[u32]를 가져와서 슬라이스 내장 메소드를 수행한다!
        
     */

}
fn show(vec:&Vec<String>){
    for v in vec{
        println!("{}",v);
    }
}
fn shows(array:&[u32]){
    for v in array{
        println!("{}",v);
    }
}
fn facto(n:u32)->u32{
    (1..n+1).product()
}