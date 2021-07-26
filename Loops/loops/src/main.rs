fn main() {
    // This will print forever
    // can use break to esc loop
    loop{
        println!("printing");
        break;
    }

    /*
     * Loops until counter is 10
     * Returns counter
     * Breaks can be used as returns
     */
    let mut counter = 0;
    let result = loop{
       counter += 1;

       if counter == 10 {
           break counter;
       }
    }

    counter = 10;
    while counter != 0{
        println!(counter);
        counter -= 1;
    }

    // We can also make some Pythonic loops
    let a = [10, 20, 30, 40, 50];
    for num in a.iter(){
        println!("curr value is {}", num);
    }

    // prints 10, 9, 8, .., 1
    for num in (1..10).rev(){
        println!(num);
    }
}
