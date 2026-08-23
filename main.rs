// dong this to always inspect files binaries
//
fn main(){
    let result = authenticate("password123");
    println!("this is {}", result);

}

fn authenticate(password: &str) -> bool {
    password == "password123"


}


// fn main() {
//     let result = authenticate("password123");

//     println!("{}", result);
// }

// fn authenticate(password: &str) -> bool {
//     password == "password123"
// }
