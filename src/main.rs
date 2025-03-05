

//NOTE: The code below is just for practicing lifetimes.

fn main() {
    println!("Hello, world!");
    init();

}

struct Test1 {
    name: String,
    val: i32
}
struct Test2<'a> {
    name: &'a str,
    val: &'a i32
}

/* initial functions */

fn init () {
    let a = 2;
    let b = 3;
    let mut c = sum(a, b);
    let mut d = sum(a, b);
    println!("{c} - {d}");
    c = sum_lifetime(&a, &b);
    d = sum_lifetime(&a, &b);
    println!("{c} - {d}");

    let a = "adsf".to_string();
    let b = "qwerty".to_string();
    let mut c = concat(a.clone(),b.clone());
    let mut d = concat(a.clone(), b.clone());
    println!("{c} - {d}");
    let a = "asdf";
    let b = "qwerty";
    c = concat_lifetime(a,b);
    d = concat_lifetime(a, b);
    println!("{c} - {d}");
}


//integers
fn sum(a: i32 , b: i32 ) -> i32 {
    return a + b;
}
//strings
fn concat(a: String, b: String) -> String {
    format!("{a} {b}")
}
//structs

/* lifetime based functions */

//integers 
fn sum_lifetime<'a>(a: &'a i32, b: &'a i32) -> i32 {
    *a + *b
}

//strings
fn concat_lifetime<'a>(a: &'a str, b: &'a str) -> String { //because the input can be dynamic I do not think we can create a return with a lieftime
    let tmp = format!("{a} {b}");
    tmp
}

//structs