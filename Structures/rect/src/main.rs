#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

/*
 * Similar Syntax to Python.
 * Using impl so specifit that this will be attached
 * to Rectangles only. We had to use &self because
 * Methods can still take ownership like functions do.
 */

/*
 * We can declare as many methods as we want.
 * Methods can take self as a parameter,
 * Self and other params
 * Or not use self param at all.
 * Functions that don't use self are called Associated
 * functions. To call associated functions we use
 * ::
 *
 * Also we can have multiple impl blocks. for the same
 * Struct type. Usefulness of this will be explained later.
 */ 
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,        
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    /*
     * :? Means to print in Debug mode.
     * Primitive types implement something known as
     * Display. Struct's dont. Since println! doesn't
     * know how we ant to display rect it gives an error
     * Speicfying debug helps but we need to add
     * #[derive(Debug)] this allows compiler to 
     * implement Debug for structs instead of doing it
     * ourselves. Using {:#?} adds some styling
     */
    println!("rect1 is {:#?}", rect1);
    println!("rect1 area is {}", rect1.area());
}

/*
 * fn to calculate Area of a Rectangle. This is fine
 * But since this function can only be used by Rects
 * We can instead make it a Rectangle method
 *
fn area(rect: &Rectangle) -> u32{
    rect.width * rect.height
}
*/
