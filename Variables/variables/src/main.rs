fn main() {

    /*
     *This wont work, By default all variables are immutable
     *
     */

    /*
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // this causes the error 
    println!("The value of x is {}",x);
    */

    // mut allows vars to be mutable
    let mut x = 5;
    println!("The value of x is {}",x);
    x = 6;
    println!("The value of x is {}",x);

    /*
     *Pros and cons:
     *  Using large DS mutating in place is faster than copying
     *  In smaller DS creating new instances is safer and may provide more
     *  more clarity
     */
     
     /*
      * Immutable Vs Constant
      *     mut cant be used with constants
      *     both immu and constants cant be changed
      *     const is used to declare constants instead of let and must
      *     annotate type
      *     Constants can be declared in any scope including global
      *     Constants may only be set only to a constant expression
      * 
      */

    const MAX_POINTS: u32 = 100_000;


    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is {}",y);

    /*
     * The above is known as shadowing. We can declare a new var
     * with the same name as a previous one. "The first var is being
     * shadowed by the second var"
     *
     * Shadowing is different from mut.
     * We can change the type of the var when shadowing but not when
     * mut. This is because shadowing reconstructs the var
     */

    let spaces = "   ";
    let spaces = spaces.len();

}
