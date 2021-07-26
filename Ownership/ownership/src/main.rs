
/*
 * Usually 2 types of languages. Garbage collecting and explicit allocation
 * Rust uses neither it has a third approach. Memory management via 
 * ownership
 *
 * Ownership Rules
 *      
 *      String Types
 *          We have String literals represented as 
 *              let s = "stuff";
 *          This type of string exists within the scope it was defined in
 *          and is stored on the Stack. The String is also immutable
 *
 *          We also have The String Type. Allocated on the heap and is
 *          mutable. 
 *              let s2 = String::from("hello world");
 *          Useful when we don't know the size of the string needed
 *          at compile time. Example is when we are waiting for user
 *          inputed strings from the user
 *
 *          :: is an operator that allows us to namespace a particular
 *          function from the String class. (Similar to :: in C++)
 *
 *
 *      Memory and Allocation
 *          String literals are faster and more efficient compared to
 *          String Types. This is because we know the contents during
 *          compile time and because of this we know it will be stored
 *          on the Stack.
 *
 *          String Types are stored on heap and we need to do work
 *          to allocate some memory. This is because the string size
 *          is unknown at compile time.
 *              Mem must be requested during runtime.
 *              Must return/free mem when done.
 *
 *          In Rust memory is auto returned/freed once the var is out of scope.
 *          When mem is out of scope Rust calls the drop() function.
 *          When using = on heap allocated memory, the data is moved
 *          (C++ move) to the other variable. This solves the double free
 *          problem. Rust prevents you from using the old var.
 *
 *          Stack-Only Data Copy
 *              We don't need to use things like clone when copying stack
 *              allocated vars because A we know the data at compile time
 *              and B Copying on the Stack is really fast so on the Stack
 *              There is no diff between shallow and deep copies.
 *
 *              Rust has special annotation called Copy trait that can be
 *              implemeted for different types. If a type implements copy
 *              trait than an older var can still be used even after "move"
 *              Cant implement copy trait if a type or any of its parts
 *              implements the drop trait
 *
 *              Some Types that implement copy:
 *                  All Integers
 *                  Booleans
 *                  floating nums
 *                  char
 *                  Tuples (if they only contain types that implement copy)
 *      
 *      Ownership and Functions
 *          Passing vals to a function is similar to assigning a value to a
 *          var. Specifically vals will their be moved or copied.
 *
 *          This all depends on whether or not copy trait is implemented.
 *          String Types by default will be "moved" into the function
 *          that called it first and we wont be able to use it after that
 *          point.
 *
 *          Integers can be copied into functions and we still can use the
*           original var that was passed.
*       
*       Return Values and Scope
*           returning vals also transfers ownership.
*           Moving and losing vars when passing them into functions is
*           tedious. A tedious fix is to return a tuple
*           (original_data, result) but again this is also tedious
*           references fix this issue.
 *
 */

fn main() {
    println!("Hello, world!");
    let s = "Hello World"; // This is a string literal

    let mut s2 = String::from("Hello World"); // This is a String Type
    s2.push_str("!");
    println!("{}",s2);

    // Simple case. Num2 makes own copy of num1.
    // Stack Copy
    let num1 = 5;
    let num2 = num1;

    // C++ equivalent of a move
    // String model looks like the C++ String Class
    // = operator creates a shallow copy (returns pointer to heap) but
    // doesn't actually make a new copy of string1. string1 becomes invalidated
    //
    let string1 = String::from("Hello world");
    let string2 = string1;

    // This would cause an error for trying to use an invalidated ref
    // println!(string1);
    println!("{}",string2); // This works fine

    //Deep Copy Example
    let original = String::from("This is the original");
    let copy = original.clone(); // makes a deep copy of original

    let s = String::from("data");
    // we can no longer use s it has been moved...

    let num = 1;
    make_copies(num);
    // we can still use num since integers implement copy trait
    
    let new_owner = give_ownership(); // data's ownership belongs to new_owner
    let some_string = String::from("hello"); 
    // some_string moved into a_soul_for_soul, creates new string and
    // returns ownership of that new string
    let perfect_balance = a_soul_for_soul(some_string);

}

fn give_ownership() -> String{
    let some_string = String::from("data"); // create a String obj
    some_string //move it when returning
}

fn a_soul_for_soul(s: String) -> String{
    s
}

fn make_copies(n: i32){
    println!("{}",n);
}

