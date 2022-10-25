fn main() {
    // scalar
    //
    //
    // Signed (i8) and Unsigned (u8) and floating point (f32 or f64)

    //let x = 2.0; // f64

    //let y: f32 = 3.0; //

    let heart_eyed_cat = '😻';

    println!("{heart_eyed_cat}");

    // bools
    // chars
    //
    //

    //
    // compound
    //
    //
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // with optional type notations
                                             // Tuple can contain many different types
                                             //
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    println!("The value of y is: {}", { tup.1 });

    let _unit = (); // unit is ()
                    //

    // the array type
    //
    let a = [1, 2, 3, 4, 5, 6]; // array is like a tuple but every element must have the same type
                                // Fixed length

    const months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [2, 3, 4, 5, 6];

    let a = [3; 5]; // this equals [3, 3, 3, 3, 3]
                    //

    let first = a[0];
    let second = a[1]; // etc

                    
}
