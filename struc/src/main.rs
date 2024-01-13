fn main() {
    let mut tu = (13,32);

    let x = &mut tu;
    //let y = &mut tu; -> you can't borrow tu more once at a time.


    *x = (1,2);

    //아래처럼 소유 트리를 통해서 접근하는거는 가능해 
    let y = &mut x.0; //. 연산자는 알아서 체이닝해서 드러감
    //let z = &mut x.0; -> 동일한거 borrow 안댐!

    *y = 1;

    let mut count = 0u32;

    let mut k1 = 1;
    let k = loop
    {
        k1 += 1;

        if k1 == 11
        {
            break k1;
        } 
    };

    println!("{}",k1);

}
