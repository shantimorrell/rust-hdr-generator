use std::process::Command;

// fn print_args (args: &Vec<String>) {
//     for i in args {
//         println!("{:}", i);
//     }
// }



fn main () {
    // Gather command line args
    let args: Vec<_> = std::env::args().collect();

    // Path to hdrgen executable
    const HDRGEN_PATH: &str = "../../hdrgen_macosx/bin/hdrgen";
    
    // Used for printing some debug info
    const DEBUG: bool = false;


    //let path_to_input_files = "../../HDRICalibrationTool/examples/inputs/input_images/";
    //let path_to_response_function = "../../HDRICalibrationTool/examples/inputs/parameters/response_function_files/";

    // Path to write output HDR image
    let output_path = "test.hdr";

    // Check there's command line args
    if args.len() > 1 {
        // Input files are args at indices [1, args.len() - 1)
        let input_files = &args[1..(args.len() - 1)];

        // Response function arg is last command line arg
        let response_function_path = &args[args.len() - 1];

        if DEBUG {
            println!("\nInput files: \n{:?}", input_files);
            println!("\nResponse function path: \n{:?}", response_function_path);
        }

        // Create a new command for hdrgen
        let mut command = Command::new(HDRGEN_PATH);

        // Add input LDR images as args
        for input_file in input_files {
            command.arg(format!("{}", input_file));
        }

        // Add output path for HDR image
        command.arg("-o");
        command.arg(format!("{}", output_path));

        // Add camera response function
        command.arg("-r");
        command.arg(format!("{}", response_function_path));

        // Add remaining flags for hdrgen step
        command.arg("-a");
        command.arg("-e");
        command.arg("-f");
        command.arg("-g");

        // Run the command
        let status = command.status().unwrap();
        
        if DEBUG {
            println!("\nCommand exit status: {:?}\n", status);
        }
        
        // Check if hdrgen command was successful
        if status.success() {
            println!("Will eventually return output path: {}", output_path);
        }
        else { 
            println!("Error, non-zero exit status. hdrgen command failed.");
        }
    }
    else {
        println!("Error. No args were entered.");
    }
}