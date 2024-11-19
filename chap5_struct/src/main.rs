#[derive(Debug)]
struct User{
    active: bool,
    username:String,
    email:String,
    sign_in_unit:u64,
}


// fn main(){
//     let user=User{
//         active:true,
//         username:String::from("yazib"),
//         email:String::from("yazib@gmail.com"),
//         sign_in_unit:1,
//     };

//     println!("{:?}",user);

//     let user1=User{
//         active:true,
//         username:String::from("yazib"),
//         email:String::from("yazib@gmail.com"),
//         sign_in_unit:1,
//     };

//     println!("{:?}",user1);
// }

/* =========================================== */
// fn main(){
//     println!("{:?}", build_user());
// }

// fn build_user()->User{
//     User{
//           active:true,
//           username:String::from("yazib"),
//           email:String::from("yazib@gmail.com"),
//           sign_in_unit:1,
//     }
// }


/* ============================================ */

// fn main(){
//     let s=String::from("yazib");
//     // println!("{:?}", build_user(s));

//     let user1=User{
//         email:String::from("another@example.com"),
//         ..build_user(s)
//     };
//     println!("{:?}", user1);
// }

// fn build_user(username:String)->User{
//     User{
//           active:true,
//           username,
//           email:String::from("yazib@gmail.com"),
//           sign_in_unit:1,
//     }
// }


/* ========================================================= */
// #[derive(Debug)]
// struct  Color(i32,i32,i32);

// fn main(){
//     let black=Color(0,1,2);
//     println!("{:?}",black);
//     println!("{} | {}",black.0 , black.1)
// }

/* ======================================================== 5.2========================= */
// fn main(){
//     let rec1=(2,2);

//     print!("Value {}",area(rec1));

// }
// fn area(dimension:(u32,u32))->u32{
//     dimension.0*dimension.1
// }

/* ======================================================= */
// #[derive(Debug)]
// struct Rectangle{
//     width: u32,
//     height: u32,
// }

// fn main(){
//     let rec1=Rectangle{
//         width:5,
//         height:5,
//     };
//     println!("check {rec1:?}");
//     println!("Area {}",area(rec1));
// }

// fn area(dimension:Rectangle)->u32{
//     dimension.width*dimension.height
// }

// ==============================================
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn main(){
    let scale=5;
    let rec1=Rectangle{
        width:dbg!(2*scale),
        height:5,
    };
    dbg!(&rec1);
    println!("check {rec1:?}");
    
}


