
/* ===============OWNERSHIP================== */
/* fn main() {
    let s="hello";
    println!("{s}");

    let s =String::from("Yazib");
    println!("{s}");

    let mut s =String::from("Yazib");
    s.push_str(" here");
    println!("{s}");
}
 */

 /* ============================== */
//  fn main(){
//     let a=5;
//     let b=a;

//     println!("{a}");
//     println!("{b}");

//     let mut s =String::from("Yazib");
//     s.push_str(" string");
//     let s1=s;

    
//     println!("{s1}");

    
//  }

/* =================================== */
/* fn main(){

    let mut s =String::from("Yazib");
    s.push_str(" string");
    let s1=s.clone();
    let s2=s1;

    println!("{s}");
    println!("{s2}");

    let mut s3=s2.clone();
    println!("{s2}");
    s3.push_str(" string");
    println!("{s3}");
 }
 */

 /* ===============function ownership ================ */
/*  fn main(){
    let s=String::from("Muhammad yazib");

    
    let owner=takes_ownership(takes_ownership(s));
    println!("{owner}")
    
 }

 fn takes_ownership(name:String)-> String {
    /* println!("take ownership {name}"); */
    format!("Owner {name}")
 } */


  /* ============================= */

/* fn main(){
    let s1=gives_ownership();
    println!("s1 = {s1}");
    
    let s2=takes_and_givesback(String::from("hello"));
    println!("s2 = {s2}");

    let a=gives_ownership();
    let s3=takes_and_givesback(a);
    println!("s3 This is = {s3}");
    // println!("s3 This is = {a}");
}

fn gives_ownership()->String{
    let some_string=String::from("Yazib");
    some_string
}

fn takes_and_givesback(a_string:String) -> String{
        a_string
} */









/* =====================REfrences and Borrowing ====================== */
// fn main(){
//     let s1=String::from("Yazib");
//     println!("{}",calculate_length(/* &s1 */ &String::from("Muhammad")))
// }

// fn calculate_length(s:&String)-> usize{
//     s.len()
// }




// ===========================================
/* fn main(){
    let mut  s = String::from("Yazib");
    println!("{}",change(&mut s));
    println!("{s}");
}
fn change(a:&mut String)->&String{
    a.push_str(" string");
    a
    
} */

// ===================================
// fn main(){
//     let mut  s = String::from("Yazib");

//     let r1=&s;
//     println!("s={s} \nr1={r1} ");
//     let mut r2=&s;
//     r2=&r1;

//     // r2.push_str("string");
//     println!("s={s} \nr1={r1} \nr2={r2}");

// }


// ========================================
// fn main(){
//     let mut s=String::from("Hello");

//     let r1=&s;
//     let r2=&s;

//     // println!("r1={r1} \nr2={r2}");
    
//     println!("s={s} \nr1={r1} \nr2={r2}");
//     let mut r3=&mut s;
//     r3.push_str(" string");
//     println!("r3={r3}");

// }


/* ======================================= */
// fn main(){
//     let refrence_to_nothing=dengle();
//     println!("{}",refrence_to_nothing);
// }

// fn dengle()->&String{
//     let s=String::from("Yazib");
//     &s
// }

/* ================================================ */
// fn main(){
//         let refrence_to_nothing=dengle();
//         println!("{}",refrence_to_nothing);
//     }
    
//     fn dengle()->String{
//         let s=String::from("Yazib");
//         s
//     }

/* =======================Slice============================ */
/* fn main(){
    let s=String::from("Hello world");

    let hello=&s[0..5];
    println!("{hello}");

    let hello=&s[5..11];
    println!("{hello}");

    let hello=&s[..11];
    println!("{hello}");

    let hello=&s[3..11];
    println!("{hello}");

    let hello=&s[1..];
    println!("{hello}");

    let hello=&s[..];
    println!("{hello}");

    let hello=&s[0..];
    println!("{hello}");

} */


/* ================================== */
// fn main(){
//     let s=String::from("Hello world");

//     let len=s.len();
//     println!("{len}");

//     let hello=&s[len..];
//     println!("{hello}");

// }

/* fn main(){
    let my_string=String::from("hello world");

    let a=f_word(&my_string[..5]);
    println!("{a}");

    let my_string="Hello 1234567";

    let a=f_word(&my_string[..8]);
    println!("{a}")

}

fn f_word(s:&str)-> &str{
    s
} */


/* ==================Arrays===================== */
fn main(){
    let a=[1,2,3,4,5];

    let slice=&a[3..5];
    println!("{:?}",slice);

    assert_eq!(slice, &[4, 5]);
}