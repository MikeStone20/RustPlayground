/*
 * Similar to C and C++
 * Define name of struct
 * The the types the structs will contain
 */
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    
}

fn main() {


    /*
     * Creating Struct of type User
     * The entire instance is either mutable or immutable
     * Can't have certain fields be mutable and the rest
     * immutabele.
     */
    let mut user1 = User{
        email: String::from("gmail"),
        username: String::from("name"),
        active: true,
        sign_in_count: 1,
    };

    /*
     * Can use an older struct to make a newer one
     * This is the verbose way
     */
    let user2 =  User{
        email: String::from("new@email"),
        username: String::from("newuser2"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    /*
     * Equivalent to user 2 but uses short hand ..
     * to fill in the rest of the fields
     */ 
    let user3 = User{
        email: String::from("user3@email"),
        username: String::from("username3"),
        ..user1
    };

    // Changing the email value of user1
    // This would only work if mut
    user1.email = String::from("Diff@gmail");

    struct Color(i32,i32,i32);
    struct Point(i32, i32, i32);

    /*
     * This is known as a tuple struct
     * We can make new types with whatever fields,
     * the difference being we don't need to explicitly 
     * name each field.
     */
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

}

/*
 * Function that creates User with email and usrname
 */
fn build_user(email: String, username :String) -> User{
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

/*
 * If function params are the same as the fiels names
 * we can ommit the : notation.
 */

fn better_build_user(email: String, username: String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
