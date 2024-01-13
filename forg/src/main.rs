//레퍼런스 받아서 레퍼런스로 반환
fn main()
{
    let mut k:&i32 = &23;

    {
        let parabola = vec![9,4,5,0,21,321,32];

       // k = smallest(&parabola); // do not live long error 발생!

    }
    println!("{}", k);

}

fn smallest(v:&[i32]) -> &i32
{
    // 이러면 v는 확실히 i32짜리 슬라이스임을 알 수 있다.
    // 내부 구조  v...0 ...1
    //         ->메모리 -> 길이가 몇인가
    let mut x = &v[0]; // 이건 슬라이스에서 메모리를 타고 들어간거
    let y = &v[1..];

    for i in y // 여기서 y는 &i32의 이터레이터 객체를 반환한다
    {
        if *i < *x
        {
            x = i;
        }

    }
    x
}