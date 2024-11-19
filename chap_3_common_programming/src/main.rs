/* ===================================3.1 Variables======================================== */

/* fn main() {
    let x=5;
    println!("the valaue of x is {x}");
    x=6;
    println!("the valaue of x is {x}");
}
 */
// ==================================================
/* fn main() {
    let mut x=5;
    println!("the valaue of x is {x}");
    x=6;
    println!("the valaue of x is {x}");
}
 */

// ===================================================
/* 
fn main() {
    let  x=5;

    let x=3;

    {
        let x=x*2;
        println!("The value of inner scope x {x}");

    }

    println!("The value of global scope x {x}");
} */


// ================================================
/* fn main(){
    let space="   ";
    let spaces=space.len();

    println!("space of the string {spaces}");

    let name_space="Yazib";
    let spaces=name_space.len();

    println!("space of the string {spaces}");
} */

/* ====================================================================3.2 Data Types======================================== */
/* fn main(){
    let a:u32="42".parse().expect("NOt a number");
    println!("string convetrted into Number {a}");

    let a:u32="ALi".parse().expect("NOt a number");
    println!("string convetrted into Number {a}");

} */


// ==================Floating Point============
/* fn main(){
    let x=2.0;
    let y:f32=3.0;

    println!("THis is f64 by defaut {x}");
    println!("THis is f32 by defaut {y}")
} */

// ==================== Numeric Opertions==========
/* fn main(){
    let sum=5+10;
    println!("sum value : {sum}");

    let difference=9.5-5.5;
    println!("difference value : {difference}");

    let product=3*5;
    println!("product value : {product}");

    let div=6.0/2.0;
    let div1=-5/5;
    println!("div value : {div}");
    println!("div1 value : {div1}");

    let reminder=3/2;
    println!("reminder value : {reminder}");
} */

// ============================================
/* fn main(){
    let t=true;
    let f: bool=false;
    println!("t: {t} | f:{f}");
} */

// ===============================================
// tuple
/* fn main(){
    let tuple=(1,2,3);

    let mul_tiple=(500, 4.3 , false);
    let (x,y,z)=mul_tiple;

    println!("x={x} y={y} z={z}");

} */


/* ======================================================= */
/* fn main(){
    let mul_tiple=(500, 4.3 , false);
    let (x,y,z)=mul_tiple;

    let five_hundred=mul_tiple.1;
    println!("{five_hundred}");

     let six_point_four=mul_tiple.2;
     println!("{six_point_four}");

} */

// ========================================================
/* fn main(){
    let a=[1,2,3,4,5];
    let a:[i32; 3]=[1,2,3];

    println!("{:?}",a);

    let b=[3;4];
    println!("{:?}",b);
} */

// // ==========================================
// use std::io;

// fn main(){
//     let a=[1,2,3,4,5];

//     println!("please enter an array index");
//     let mut index=String::new();

//     io::stdin().read_line(&mut index).expect("Failed to read line");

//     let index:usize=index.trim().parse().expect("Index is not a number");

//     println!("{:?}",a[index]);
// }


/* =================================functions============================================================================= */
/* fn main(){
    another_fn();
}

fn another_fn(){
    println!("Hello new fn");
}
 */

/* ============================= */
/* fn main(){
    another_fn(5);
}

fn another_fn(x:i32){
    println!("Hello new fn value {x}");
} */

/* ============================== */
/* fn main(){
    another_fn(5,'h');
}

fn another_fn(x:i32,y:char){
    println!("Hello new fn value {x} {y}");
}
 */

/* ==================================== */
/* fn main(){
    another_fn([1,2,3,4,5]);
}

fn another_fn(x:[i32;5])->[i32;5]{
    println!("Hello new fn value {:?}",{x});
    x
} */

/* ======================================== */
/* fn main(){
    let x={
        let y=5;
        y
    };
    println!("{x}")


} */



/* ===================================== */
/* fn main(){
    let number=4;

    if number<4{
        println!("less then 4");
    }else if  number==4{
        println!("Equal to 4")
    }
    else {
        println!("greater then 4")
    }
} */

/* ======================================= */
// fn main(){
//     let number=10;

//     if number !=0{
//         println!("number was something other then 0")
//     }else {
//         println!("it's a 0")
//     }
// }


/* ============================= */
// fn main(){
//     let condition=5;
//     let a= if condition==5 { 5 } else { 6 };
//     println!("{a}");
// }


/* =============== */
// fn main(){
//     let mut a=0;
//     loop{
//         if a <=5{
//             println!("{a}");
//         }
//         a+=1;
        
//     }
// }

/* ========================= */
// fn main(){
//     let mut counter =0;

//     let result=loop{
//         counter += 1;
//         if counter==10{
//             break counter*2
//         }
//     };

//     println!("{result}");

// }

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}