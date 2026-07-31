
pub mod if_else_concepts {
   pub fn test_if() {
    let age_to_drive: u8 = 16u8;

    println!("Enter the person's age: ");
    let myinput: &mut String = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();


    let age: u8 = myinput.replace("\n", "").parse::<u8>().unwrap();

    if age >= age_to_drive {
        println!("Issuing driver's license...");
    }
    else {
        println!("Cann't be Issued...");
    }


    //? bool
    let driver_license = if age >= 16 {true} else {false};
    println!("{driver_license}");

}
}





/*****
 //! Notes
 //? ABout the std::io
String::from("")
Creates a new, empty, owned, heap-allocated String.
String::from(...) converts a &str (string slice literal "") into an owned String.
&mut String::from("")
&mut creates a mutable reference to that newly created String.
This is necessary because read_line needs to modify the string (append the input text into it), and Rust requires explicit mutable access for that — you can't mutate data through a normal immutable reference.
let myinput: &mut String = &mut String::from("");
This stores that mutable reference in myinput.
Type annotation &mut String explicitly states: "this variable holds a mutable reference to a String."

//? std::io::stdin()
std::io — the standard library's I/O module.
stdin() — returns a handle representing the process's standard input stream (keyboard input, typically).

//? .read_line(myinput)
Reads a line of text typed by the user (up to and including the newline character) and appends it into the given String buffer (myinput).
This is why it needs a mutable reference — it's writing into existing memory rather than returning a brand-new string.
//? Note: the input captured includes the trailing \n (and possibly \r on Windows) — this matters for the next step.


//? .unwrap()
read_line returns a Result<usize, std::io::Error> — Ok(number_of_bytes_read) on success, or Err(...) if something goes wrong (e.g., I/O failure).
.unwrap() is a quick way to say: "give me the success value, and if it's an error, just crash the program (panic!)." It's convenient for learning/small programs but considered poor practice for production code, where you'd handle the Result properly (e.g., with match or ?).

//? myinput.replace("\n", "")
 **.replace(pattern, replacement)**
 -> returns a new String with all occurrences of "\n" (newline) replaced with "" (nothing) — effectively removing newline characters.
This is necessary because read_line includes the newline the user typed when hitting Enter, and that character would break numeric parsing otherwise.
(Minor note: on Windows, input often has \r\n, so a more robust approach is usually .trim() instead of .replace("\n", ""), since .trim() removes whitespace — spaces, tabs, \r, \n — from both ends. But .replace works fine here for typical Unix-style input.)
//? .parse::<u8>()
//? .parse()
 is a generic method that attempts to convert a string into another type — here, explicitly specified as u8 via the turbofish syntax ::<u8>.
Returns a Result<u8, ParseIntError> — Ok(value) if the string was successfully parsed as a valid number in range, or Err(...) if it wasn't (e.g., user typed letters, or a number bigger than 255).

//? .unwrap()
Same as before — extracts the successful u8 value, or panics if parsing failed.


 * 
 */