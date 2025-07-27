struct User<'a> {
    // This tells the compiler: the reference `name` inside `User`
    // must not outlive the lifetime `'a`.
    name: &'a str,
}

fn main() {
    // We create a `String` on the heap.
    let first_name = String::from("Tushar");

    // We create a reference to that string and store it inside the struct.
    // This is safe because both `user` and `first_name` live in the same scope.
    let user = User { name: &first_name };

    // This is valid because `first_name` is still in scope here.
    println!("The name of the user is: {}", user.name);
}