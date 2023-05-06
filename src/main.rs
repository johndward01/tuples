fn main() {
    let tuple = (1.0, 2, 2.22);
    let (x, y, z) = tuple;
    println!("The value of x: {x}");
    println!("The value of x: {y}");
    println!("The value of x: {z}");

    println!(" ");

    let first_tuple_value = tuple.0;
    let second_tuple_value = tuple.1;
    let third_tuple_value = tuple.2;

    println!("The value of index 0 is: {first_tuple_value}");
    println!("The value of index 1 is: {second_tuple_value}");
    println!("The value of index 2 is: {third_tuple_value}");
}
