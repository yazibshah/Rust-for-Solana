/* ============ENUMS==================== */
// #[derive(Debug)]
// enum  IpAddrKind {
//     V4,
//     V5
// }

// fn main(){
//     let four=IpAddrKind::V4;
//     let five=IpAddrKind::V5;

//     println!("{:?}",four);
//     println!("{:?}",five);
// }


// /* ============================ */
// #[derive(Debug)]
// enum  IpAddrKind {
//     V4,
//     V5
// }

// #[derive(Debug)]
// struct  IpAddr{
//     kind:IpAddrKind,
//     address:String
// }

// fn main(){
//     let home=IpAddr{
//         kind:IpAddrKind::V4,
//         address:String::from("Yazib")
//     };
    
//     dbg!(home);
// }

// /* ============================ */
// #[derive(Debug)]
// enum  IpAddrKind {
//     V4(String),
//     V5(bool)
// }

// #[derive(Debug)]
// struct  IpAddr{
//     kind:IpAddrKind,
//     address:String
// }

// fn main(){
//     let home=IpAddr{
//         kind:IpAddrKind::V4(String::from("value")),
//         address:String::from("Yazib")
//     };
    
//     dbg!(home);
// }



// /* ============================ */
// #[derive(Debug)]
// enum  IpAddrKind {
//     V4(String, i32 , bool),
//     V5(bool)
// }

// #[derive(Debug)]
// struct  IpAddr{
//     kind:IpAddrKind,
//     address:String
// }

// fn main(){
//     let home=IpAddr{
//         kind:IpAddrKind::V4(String::from("value"),3,true),
//         address:String::from("Yazib")
//     };
    
//     dbg!(home);
// }

/* /* ============================ */
#[derive(Debug)]
enum  IpAddrKind {
    V4(String, i32 , bool),
    V5(bool)
}

#[derive(Debug)]
struct  IpAddr{
    kind:IpAddrKind,
    address:String
}

fn main(){
    let home=IpAddr{
        kind:IpAddrKind::V4(String::from("value"),3,true),
        address:String::from("Yazib")
    };
    
    dbg!(home);
} */

/* ============================ */
// #[derive(Debug)]
// struct Data{
//     name:String,
//     age:i32
// }
// #[derive(Debug)]
// enum  IpAddrKind {
//     V4(String, i32 , bool),
//     V5(Data)
// }

// #[derive(Debug)]
// struct  IpAddr{
//     kind:IpAddrKind,
//     address:String
// }

// fn main(){
//     let home=IpAddr{
//         kind:IpAddrKind::V5(Data{name:String::from("yazib"),age:25}),
//         address:String::from("Yazib")
//     };
    
//     dbg!(home);
// }




//===============================================================
/* #[derive(Debug)]
enum Message{
    Quit,
    Move{x:i32 , y:i32},
    Write(String),
    ChangeColor(i32 , i32 , i32)
}

impl Message {
    fn call(&self){
        println!("{} {}",self.x,self.y)
    }
}

fn main(){
    let m =Message::Move { x: 3, y: 4 };
    m.call();
    println!("M == {:?}",m);
    print!("{:?}",m.call())
} */

/* =============================================== */
/* struct QuitMessage;
struct MoveMessage{
    x:i32,
    y:i32
}

struct WriteMessage(String); */

/* ========================================================Chap 6.2 Match ============================== */
// enum Coin{
//     Penny,
//     Nickle,
//     Dime,
//     Quarter
// }

// fn value_in_cents(coin:Coin)->u8{
//         match coin {
//             Coin::Penny =>1,
//             Coin::Dime =>2,
//             Coin::Nickle=>3,
//             Coin::Quarter=>4
//         }
// }
// fn main(){
//     let m=Coin::Dime;
//     println!("{}",value_in_cents(m));
// }


/* ============================ */
/* enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter
}

fn value_in_cents(coin:Coin)->u8{
        match coin {
            Coin::Penny =>{
                print!("Penny Value is ");
                1
            },
            Coin::Dime =>2,
            Coin::Nickle=>3,
            Coin::Quarter=>4
        }
}
fn main(){
    let m=Coin::Penny;
    print!("{}",value_in_cents(m));
} */


// ==============================================
// enum UsState{
//     Alamba,
//     Alaska
// }

// enum Coin{
//     Penny,
//     Nickle,
//     Dime,
//     Quarter(UsState)
// }

// fn value_in_cents(coin:Coin)->u8{
//     match coin {
//         Coin::Penny =>{
//             print!("Penny Value is ");
//             1
//         },
//         Coin::Dime =>2,
//         Coin::Nickle=>3,
//         Coin::Quarter(state)=>4
//     }
// }

// fn main(){
//     let m=Coin::Quarter(UsState::Alamba);
//     print!("{}",value_in_cents(m));
// }


/* =============================================== */
// fn main(){
//     let val=plus_one(Some(5));
//     println!("{:?}",val);

//     let val=plus_one(Some(8));
//     println!("{:?}",val);
// }

// fn plus_one(x:Option<i32>)->Option<i32>{
//     match x {
//         None => None,
//         Some(5)=>Some(1),
//         _=>Some(3),
//     }
// }


/* =============================================== */
#[derive(Debug, PartialEq)] // Derive PartialEq to enable equality comparisons
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug, PartialEq)] // Derive PartialEq for UsState as well
enum UsState {
    Alabama,
    Alaska,
    // Add more states if needed
}

// fn main() {
//     let a = Coin::Quarter(UsState::Alaska);
//     let state = UsState::Alaska;
//     let b = Coin::Quarter(state.clone());

//     if a == b {
//         println!("State quarter from {:?}!", state);
//     } else {
//         println!("Not a quarter, counting it!");
//     }
// }

fn main(){
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
