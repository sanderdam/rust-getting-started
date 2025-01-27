pub fn numeric_types(){
    // Sometimes we need to explicitly tell the compiler the type of a variable
    let guess: u32 = "42".parse().expect("Not a number!");

    // Numeric types

    let mut eightSigned: i8 = 8;
    let mut eightUnsigned: u8 = 8;
    eightSigned = 127;
    //eightSigned = eightSigned + 1; // Error: literal out of range for i8

    //Floating points
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}