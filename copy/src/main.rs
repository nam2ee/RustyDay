use std::collections::HashMap;
type Table = HashMap<String,Vec<String>>;
fn main() {
    let mut table = Table::new();
    table.insert("yoonjae".to_string(),vec!["a".to_string(),"b".to_string()]);
    table.insert("meggie".to_string(),vec!["a".to_string(),"b".to_string()]);

    let m = &table;
    printMap(m);

    let x = 10;
    let k = &x;
    assert!(*k==10);

    let mut v = vec![1255,455];
    v.sort(); // 메서드 호출 시, 자동으로 레퍼런스를 가져오도록 되어있다!

}

fn printMap(table: &Table){
    for (artist,works) in table
    { // recursive하게 참조를 생성하도록 되어있다!!
        println!("{}",artist);
        for work in works{
            println!("{}",work);

        }
            

    }
        
}
