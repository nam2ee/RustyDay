fn main() {
    /*
    Vector type의 Under the Hood analyze
     */
    let mut k:Vec<String> = Vec::new();
    println!("{}", k.capacity()); //0
    k.push("go".to_string());
    println!("{}", k.capacity()); //4
    k.push("go".to_string());
    k.push("go".to_string());
    k.push("go".to_string());
    println!("{}", k.capacity()); //4 
    k.push("go".to_string());
    println!("{}", k.capacity()); // 8

    let x:Vec<String> = Vec::with_capacity(8);
    //
    println!("{}", k.capacity()); // 8
    // 가능하면 러스트에서는 라이브러리들이 with_capacity로 호출하려고 한다

    //예를 들면, 다음과 같은 매크로는 컴파일 시점에서 capacity를 알기 떄문에..
    let mut k = vec![1,2,3,4];
    println!("{}", k.capacity()); //4 딱 맞춰서
    k.push(5); // 근데 사실상 mut이 아니라면 매우 효율적으로 관리하는 것은 자명하다
    println!("{}", k.capacity()); //8

    let k : Vec<u32> = (0..5).collect(); //collect로 할 수 있는건 많으니까 타입을 명시해
    //이러한 경우 <이터레이터의 값>을 넣어서 Vector를 만들 때도
    // 당연하게도 컴파일러가 크기를 감지할 수 있기에 with_capacity

    //벡터에서 어느 위치에서든 요소를 제거하거나 뺼 수 있다
    
    let mut v = vec![10,20,30,40,50];
    v.insert(3,35);

    v.remove(1);
    assert_eq!(v,[10,30,35,40,50]);

    //하지만 이것들은 영향을 받은 요소 뒤 모두에게 영향을 미친다
    // maybe sequential list로 구현한 듯 하다 -> 확인해볼 것


    
}