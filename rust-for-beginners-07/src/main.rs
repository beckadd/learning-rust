fn main() { 
    let x = 5;
    // compiler error if we assert x = 6 - we can't mutate the value as-is
    
    //"shadow" a variable:
    //Declare a new variable with the same name as the previous variable, creating a new binding

    let x = x + 1; // works, because now we're redefining the variable in terms of itself.

    //define a mutable variable - this is how we get a variable to truly "vary"
    let mut y = 6;
    y = 3;

    //declare constants:
    const SCORE_LIMIT: u32 = 100;
    

    //bunch of squigglies because rust-analyzer helpfully notes that there are some unused variables.
}
