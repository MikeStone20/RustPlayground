/*
 * If statements work similar to that of Python
 * keyword if followed by statement
 */
fn main() {
    println!("Hello, world!");
    let num = 3;

    if num < 5{
        println!("Less than 5");
    }else if num % 2 == 0{
        println!("Number is even");
    }else{
        println!("More than 5");
    }

    // This would work in C and Python but not in Rust
    /*
     * if num {
     *      println!("do something");
     * }
     */

    //if statements are expressions we can use it with let
    let x = if num == 2 {2} else {6};
    // let x = if num == 2 {2} else {"six"}; // this won't work need to be same types for return
}
