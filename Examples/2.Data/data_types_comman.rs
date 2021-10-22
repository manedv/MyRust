fn main() {
	
	
	//boolean
	let isfun:bool = true;
	println!("Is Rust Programming Fun ? {}",isfun);
	
	//==================================================================
	//Number Separator for just writing in better way
   let float_with_separator = 11_000.555_001;
   let int_with_separator = 50_000;
   
   println!("float value {}",float_with_separator);
   println!("int value {}",int_with_separator);
   
   //output
   //float value 11000.555001
	//int value 50000
	
	//=======================================================
	// Overflow
	
	let height:u8;
	height = 257;   //overflow value is 1
	 println!("height is {}",height);
	 
	 
}