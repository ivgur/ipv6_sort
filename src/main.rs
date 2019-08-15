extern crate clap;
use std::env;
use std::fs;
use std::net::Ipv6Addr;
use std::collections::BinaryHeap;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Write;
use clap::{Arg, App, ArgMatches};

//Return sorted ipv6 addresses as source use file
//temporary util get two args, then will be added proper process of second argument 


struct Config {
    input_file: String,
    output_file: String,
}

//fn parse_config(args: &[String]) -> Config {
//    let input_file = args[1].clone();
//    let output_file = args[2].clone();
//
//    Config {input_file, output_file}
//}


fn main() {
  let arguments = parse_arguments();
//	let args: Vec<String> = env::args().collect();
//	let config = parse_config(&args);
let config = Config {
                      input_file: arguments.value_of("input").unwrap().to_string(),
											output_file: arguments.value_of("output").unwrap().to_string()
										};

	let mut sort_heap = BinaryHeap::new();
	let contents = fs::read_to_string(config.input_file)
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
		match config.output_file.as_str() {
		"stdout" =>  for num in sort_vec {
       println!("{:?}", Ipv6Addr::from(num))

},

_ => {for num in sort_vec {
			let path = Path::new(&config.output_file);
			let display = &path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&config.output_file) {
        Err(why) => panic!("couldn't create {}: {}", &display, why.description()),
        Ok(file) => file,

};

    match file.write_all(Ipv6Addr::from(num).to_string().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", &display, why.description()),
        Ok(_) => println!("successfully wrote to {}", &display),

}
	   }},

}
}

fn parse_arguments() -> ArgMatches<'static> {
    App::new("ipv6_sort")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("sort ipv6 addresses, not support ipv6 addresses with mask")
				.arg(Arg::with_name("input")
             .short("i")
             .long("input")
						 .required(true)
             .value_name("INPUT FILE")
						)
				.arg(Arg::with_name("output")
             .short("o")
             .long("output")
						 .required(true)
             .value_name("OUTPUT FILE")
						)
        .get_matches()
}
