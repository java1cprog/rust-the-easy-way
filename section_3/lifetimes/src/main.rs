
struct TestStructure<'a>{
	x: &'a u32 
}

impl<'a> TestStructure<'a>{
	fn return_x(&self) -> &'a u32{
		self.x
	}
}

fn main() {
    let ts = TestStructure{
    	x: &5
    };

    println!("{:?}", ts.x);
}
