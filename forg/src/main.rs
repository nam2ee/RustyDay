static mut STASH: &i32 = &128;
static okx:&i32 =  &456;
fn main() {
    println!("Hello, world!");

    let x = &okx;
    p(&x);

    unsafe{
        println!("{}",&&&&STASH); //chaining
    }

}


fn p(x:&'static i32){

    unsafe{
        STASH = x;
        // 이 때 참조를 전달하고 죽는 것이다
        // static과의 라이프사이클이 안 맞음
    }    
}