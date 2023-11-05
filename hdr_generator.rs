fn print_args (args: &Vec<String>) {
    for i in args {
        println!("{:}", i);
    }
}





fn main () {
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 1 {
        println!("Some args were entered!");

        println!("All args: ");
        print_args(&args);


        println!("Args 1 through len - 1: ");
        print_args(&args[1..(args.len() - 1)].to_vec());

        let input_files = &args[1..(args.len() - 1)];        //.to_vec();
        println!("Input files: {:?}", input_files);


        let output_path = &args[args.len() - 1];
        println!("Output path: {:?}", output_path);

        



    }
    else {
        println!("No args were entered.");
    }




    println!("\nThis will eventually be the output path...");
}