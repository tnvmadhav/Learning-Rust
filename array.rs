fn main() {
	let my_array: [i32; 4] = [1,2,3,4];
	for x in my_array.iter() {
		println!("The array element is {:?}", x); 
	}
}
