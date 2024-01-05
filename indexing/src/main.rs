fn main() {
    /* 
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    let v1 = v[1];
    let v2 = v[2];
     */
    let mut v = vec!["101" , "102", "103"];
    let v1 = v[1];
    let v2 = v[2];

    let fifth = v.pop().expect("empty");
    //assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    //assert_eq!(second,"102");

    let s = "substitue".to_string();

    //assert_eq!(third,"103");

    //assert_eq!(v,vec!["101","104","substitue"]);

    struct Person {name: Option<String> , age: u32}

    let mut composers = Vec::new();

    composers.push(Person{
        name: Some("yoonjae".to_string()),
        age: 32
    });

    let first_name = composers[0].name.take();
}
