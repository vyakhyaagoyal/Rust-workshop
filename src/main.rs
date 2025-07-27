// fn main() {
//   for i in 0..10{
//     let c=do_mul(i,10);
//     println!("The value of c is: {}", c);
//   }
// }

// fn do_mul(a:i32,b:i32)->i32{
//     return a*b;
// }

fn main() {
  let a: i32 = 10;
  let b = 20;';
  let sum = do_sum(a, b); // Call the function with arguments
  println!("Sum of {} and {} = {}", a, b, sum);
}

fn do_sum(a: i32, b: i32) -> i32 { // Define a function named do_sum
  return a + b; // Return the sum of a and b
}