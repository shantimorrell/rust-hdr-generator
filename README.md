# rust-hdr-generator

Standalone Rust program example for running a system command to generate an HDR image. For now, just for proof of concept, the program accepts the input image locations and camera response function locations as command line arguments.

Runs a command to merge multiple LDR images into a single HDR image, saved to the file *test.hdr*:

    hdrgen LDRfiles_*.JPG -o test.hdr -r response_function.rsp -a -e -f -g

## How to Run

First, compile the Rust program using

    rustc hdr_generator.rs

Then run the program by entering

    ./hdr_generator <paths_to_input_images> <path_to_response_function>

where the command line arguments immediately following the executable name are the paths to the input images and the final argument is the path to the camera response function.
The paths to the images can be in the format "dir/*.JPG" rather than needing to list them all out individually. 


## ***Important***

In the main function, there's a variable to specify the path to the *hdrgen* executable. You'll likely need to change this to match its location on your system.

Additionally, once the program has been executed once, you'll need to delete the output file, *test.hdr*, in order to run the program subsequent times.
