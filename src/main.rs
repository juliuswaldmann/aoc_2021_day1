use std::fs::File;
use std::io::prelude::*;

//advent of code day1

fn main() -> std::io::Result<()> {

    //read the input from the file "input.txt" 
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //split content of file into a Vec<u32> of the input
    let mut split_contents = vec![];
    for s in contents.split("\r\n"){
        split_contents.push(s.parse::<u32>().unwrap());
    }

    //part one

    part_one(split_contents.clone());

    //part two 

    part_two(split_contents);

    Ok(())
}

//part one
//in part one we want to count the number of increases. 
//an increase is when a number inside of the Vec is larger than the one before it

fn part_one (split_contents :Vec<u32>) {

    //create a variable to count number of increases in the Vec
    let mut number_of_increases :u32 = 0;

    //cycle through the input numbers
    for i in 0..(split_contents.len()) {
        
        //add one to the number of increases if the current number is larger than the one before it
        //NOTE: We compare the current number against the one before it and start at index 1, so we don't compare a number to "nothing", which results in an error.  
        if i >= 1 && split_contents[i] > split_contents[i-1] {

            number_of_increases += 1;
            
        }
    }

    //print out the number of increases
    println!("part one: {}", number_of_increases);

}


//part two
//in part two we also want to count the number of increases but this time we average the compared numbers out between three numbers to reduce the "noise" 
//so an increase is when three numbers added togehter are larger than the three ones before (shiftig one to the left")

fn part_two (split_contents :Vec<u32>) {

    //create a variable to count number of increases
    let mut number_of_increases :u32 = 0;

    //cycle through the input numbers
    for i in 0..(split_contents.len()) {
        
        //add one to the number of increases if the current numbers are larger than the ones before it
        //NOTE: We compare the current numbers against the ones before it and start at index 2, so we don't compare numbers to "nothing", which results in an error
        //We also stop one index before the end so that the index does not go out of bounds
        if i > 1 && i < split_contents.len() - 1 && split_contents[i-2] + split_contents[i-1] + split_contents[i] < split_contents[i-1] + split_contents[i] + split_contents[i+1] {

            number_of_increases += 1;
            
        }
    }

    //print out the number of increases
    println!("part two: {}", number_of_increases);

}