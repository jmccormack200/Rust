fn main(){
	println!("{} days", 31);
	
	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

	println!("{subject} {verb} {predicate}",
			predicate="over the lazy dog",
			subject="the quick brown fox",
			verb="jumps");

	println!("{} of {:b} people know binary, the other half don't", 1, 2);

	println!("{number:>width$}", number=1, width=6);
	println!("{number:>0width$}", number=1, width=6);

	println!("My name is {0}, {1}, {0}", "Bond","James");

	struct Structure(i32);
	
}
