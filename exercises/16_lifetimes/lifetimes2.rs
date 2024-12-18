// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    // SOLUTION 1- Removing the string2 to the outside 
    // let string2 = String::from("xyz");
    let result;
    {
        let string2 = String::from("xyz");
         result = longest(&string1, &string2);
         // SOLUTION 2- Moving println! to inside 
         println!("The longest string is '{result}'");
    }
    // println!("The longest string is '{result}'");
}
