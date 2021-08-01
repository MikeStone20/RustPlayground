fn main() {

    /*
     * Creating a String hello world
     * hello references only the "hello" part
     * world references only the "world" part
     * This is known as a Slice. In this case a 
     * String Slice. A reference to part of a String
     */
    let s = String::from("hello world");
    // [start,end)
    let hello = &s[0..5];
    // let hello = &s[..5]; same as [0..5];
    // let world = &s[3..] from char 3 to end of string
    // let slice = &s[..] entire string
    let world = &s[6..11];

    /*
     * Different Kind of Slice
     * Type &[i32]. Stores a reference to the first elem
     * and a length.
     */
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);

}

/*
 * We use & because we dont need ownership of the
 * String just a ref will do.
 * This implementation has an issue. If string s
 * disapears there is a chance that the usize returned
 * wont. So we would be in a situation where the word
 * no longer exists but the returned value of first_word
 * still does. Synchronizing this manually is tedious
 */
fn first_word(s: &String) -> usize{
    // covert string to array of bytes
    let bytes = s.as_bytes();

    // Create an iterator to loop
    // iter: returns each element in a collection
    // enumerate: wraps results of iter in a tuple
    // First element is index second is the actual item
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    s.len()
}

/*
 * Instead of returning an index we get back a slice
 * of s. So if s disapears so does the return value
 */
fn better_first_word(s: &String) -> &str{
    let byte = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..];
}

/*
 * In Rust String Literals are Slices
 * Changing the function def to take slices
 * makes the program more general. Now we can use
 * both &String and &str with this function
 */
fn even_better_first_word(s: &str) -> &str{

    let byte = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..];
}
