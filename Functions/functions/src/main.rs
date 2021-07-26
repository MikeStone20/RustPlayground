

fn main() {
    println!("hello world");
    another_function();
}


// snake_case > camelCase
fn another_function(){

    println!("Another function");

}

// can specify params. Must be Explicitly Defined
fn func_2(x: i32, y: i32){
    println!("The value of x is {}", x);
    println!("The value of u is {}", y);
}

/*
 * Expression return values
 * Statements do not return
 * let for example is used in Statements as let doesn't return anything
 * let x = let y = 6 won't work as it would in C
 */

fn foo(){
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // no semicolon ending here. only statements use ;
    };

    // this would print 4 since {} is an expression
    println!("The value of y is {}", y);
}

/*
 * Define return type using ->
 * The value returned is the last expression
 * Can also explicitly return using return
 */
fn add() -> i32{
    2 + 5 // can also do return 2 + 5
}

/*
 * Can't add ; to the end of the line
 * This would turn the line into a statement and cause an error
 *
 */
fn increment(x: i32) -> i32 {
    x + 1 // 
}
