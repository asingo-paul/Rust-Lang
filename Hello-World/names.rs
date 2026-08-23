// Printing names in rust language
//


fn names() {
    let first_name = "Asingo";
    let second_name = "Paul";

    println!("Full name without the to_owned {}, {}", first_name , second_name);

    println!("Full name is {}", first_name.to_owned() + " " + second_name);

}

fn crush() {

    let crushy_one = "Velmah";
    let crushy_two = "Diana";
    let crushy_three = "Beatrice";

    println!("My number one {}, is better than {}, and {}", crushy_one , crushy_two , crushy_three);
}


fn main() {
    names();
    crush();
}
