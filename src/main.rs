use std::io;
use std::env;
use std::fs;
use std::net::Ipv6Addr;
use std::collections::BinaryHeap;
//Return sorted ipv6 addresses as source use file
//temporary util get two args, then will be added proper process of second argument 
fn main() {
	let mut sort_heap = BinaryHeap::new();

	let args: Vec<String> = env::args().collect();
	let input_file = &args[1];
	let output_file = &args[2];

	let contents = fs::read_to_string(input_file)
	           .expect("Something went wrong reading the file");

	for line in contents.lines() {
		let ipv6_addr: Ipv6Addr = match line.trim().parse(){
                  Ok(ipv6_addr) => ipv6_addr,
                  Err(_) => continue,
                };
		let ipv6_in_u128 = u128::from_be_bytes(ipv6_addr.octets());
		sort_heap.push(ipv6_in_u128);
   }
   let sort_vec = sort_heap.into_sorted_vec();
   for num in sort_vec {
       println!("{:?}", Ipv6Addr::from(num));
	   }
}
