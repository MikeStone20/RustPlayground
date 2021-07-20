fn main() {

    /*
     * Every Var in rust has a certain data type. If we dont explicitly
     * Specify it rust infers it.
     *
     * Rust is a statically typed langaue. It must know the types of all
     * variables at compile time. When converting, we must explicitly 
     * mention type
     */

    // explicitly mentioning type when converting
    let guess: u32 = "42".parse().except("Not a number");

    // let guess = 42.parse().except("Not a numbner"); // this wont work
    

    /*
     *Scalar Types
     *  represents a single value.
     *  4 primary Scalar Type:
     *      Integers
     *      Booleans
     *      characters
     *      floating-point
     *
     *Integer
     *  A number with no fractional component. 
     *      u32 --> unsigned int
     *      i32 --> signed int
     *  Can declare different sizes of ints
     *      i8, u8 --> 8-bit
     *      i16, u16 --> 16-bit
     *      ....
     *      i128, u128 128-bit
     *      arch --> isize, usize --> size-bit
     *          This depends on the type of computer
     *          for 64-bit then this is the max
     *  Can also make things like i57
     *
     *  Decimal --> 98_222
     *  Hex --> 0xff
     *  Octal --> 0o77
     *  Binary --> 0b1111_0000
     *  Byte(u8 only) --> b'A'
     *
     *  When integer overflows occur and we run in Debug mode. Rust Compiler
     *  will panic.
     *
     *  If compiling in --release mode it doesn't check for overflow instead
     *  we do 2's compliment wrapping. Wraps back to smallest possible val.
     *
     *  Ex: type u8. Possible max is 255. 256 wraps to 0, 257 wraps to 1.
     *
     *  Different ways to handle overflow
     *      Standard library provides some methods
     *      wrapping_* --> has methods such as wrapping_add
     *      checked_* --> Return None if the method finds an overflow
     *      overflowing_* --> return the value and a boolean if overflow found
     *      saturating_* --> set the min and max for overflows
     *
     *Floating
     *  Has 2 primitive types for floats. Numbers and Decimals
     *      f32 and f64 only (32 bit and 64 bit)
     *      Default is 64bit because it is around the same speed as f32
     *      Represented using the IEE-754 standards
     *      f32 is single point percision
     *      f64 is double point percision
     *
     *Numeric Operations
     *  +, /, -, *, %
     *
     *The Boolean Type
     *  2 values true or false (1 byte size)
     *  Speicified using bool
     *
     *The Character Type
     *  char type, and is 4 bytes long. It can represent much more than
     *  the typical ASCII table (Chinese, Japanese, Korean, Emojis)
     *
     *
     *Compound Types
     *  Can Group multiple values into one type.
     *  2 primitive compound types. Tuples and Arrays
     *
     *  Tuple
     *      A general way of grouping together values with diff types.
     *      Have fixed len cant shrink or grow
     *
     *  Array
     *      Same as tuple but every val must be of same type
     *      Use square brackets instead of para
     *      Useful when we need data allocated on the stack and not heap
     *      Unlike C we can't access beyond the size of the array will
     *      get a runtime error instead of random data
     */

    let x = 2.0; //64-bit
    let y: f32 = 3.0; //32-bit

    let t = true;
    let f: bool = false;

    //type annotations are optional
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (1, '1', "1");

    //Destructuring a tuple
    let (x,y,z) = tup;
    println!("The value of y is {}",y);

    //Acessing by index
    println!("Here is the value of x {}", tup.0); // get first val in tup

    let a = [1,2,3,4,5];
    let a: [i32; 5] = [1,2,3,4,5]; // arr of size 5 containing type i32
    let a = [3; 5]; // Array of size 5 with default value of 3
    let first = a[0]; // Acessing element at index


}
