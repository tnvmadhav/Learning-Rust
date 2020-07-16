fn main() {
    let variable_int32 = 12i32;
    let variable_int64 = 12i64;

    let mut variable_int32_mutable = 13i32;

    println!("The variable_i32 value is {}", variable_int32);
    println!("The variable_i64 value is {}", variable_int64);

    println!("The variable_i32_mutable is {}", variable_int32_mutable);

    variable_int32_mutable = 14i32;
    println!("Overriding the mutable variable of type i32");

    println!("The variable_i32_mutable is {}", variable_int32_mutable);
}
