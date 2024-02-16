use std::io;

// Define your input types:
fn custom() {
    // Enter your code here 
    
}

fn main() {
    // String created to save inputs:
    let mut inputs = String::new();
    
    // You need to read the ammount of lines based on your input type.
    io::stdin().read_line(&mut inputs);
    io::stdin().read_line(&mut inputs);

    // Once you've readed all the input lines, you need to split the string
    // and start using the values as needed. 
    let mut values = inputs.split("\n");

    // Insert the input values inside the custom fn:
    custom() 
}
