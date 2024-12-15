use std::fs;
use std::io;
use std::collections::HashMap;
use crate::calculations::degreetotal;
use crate::calculations::degreetonode;
use crate::reader::readfile;



pub mod calculations;
pub mod reader;


fn main() {

    println!("Type 'all' to see the degree distribution of the whole graph or type a positive whole number between 1 and 74 to see the top 1-74 degree frequencies.");
    let mut counter = 0;

    while counter == 0 {
        let mut input =String ::new(); 
        io::stdin().read_line(&mut input).expect("Failed to read line"); 
        let mut input = input.trim().to_lowercase();
        if input == "all" {
            let library = readfile();
            let (degreesmap, degreesvec) = degreetotal(library);
            let (probibilities, probvec) = degreetonode(degreesmap, degreesvec, 0);
            counter += 1;
        } else {
            let number: usize = input.parse().expect("Not a integer number");
            if number >= 1 && number <= 74 {
                let library = readfile();
                let (degreesmap, degreesvec) = degreetotal(library);
                let (probibilities, probvec) = degreetonode(degreesmap, degreesvec, number);
                counter += 1;
            } else {
                println!("Do not understand. Try again.");
                io::stdin().read_line(&mut input).expect("Failed to read line"); 
            }
        }
    }

}

#[test]
fn underone() {
    let library = readfile();
    let (degreesmap, degreesvec) = degreetotal(library);
    let (probibilities, probvec) = degreetonode(degreesmap, degreesvec, 0);
    for i in probvec {
        assert!(i < 1.0);
    }

}

#[test]
fn correctnumberofdegrees() {
    let library = readfile();
    let (degreesmap, degreesvec) = degreetotal(library);
    assert_eq!(degreesvec.len(), 36692);
}

#[test]
fn everynodehasedge() {
    let library = readfile();
    let (mut degreesmap, degreesvec) = degreetotal(library);
    for i in 0..36692 {
        let key = degreesmap.entry(i).or_insert(0);
        assert!(*key > 0);
    }
}

#[test]
fn correctnumberofedges() {
    let library = readfile();
    let (degreesmap, degreesvec) = degreetotal(library);
    let mut sum: u32 = degreesvec.iter().sum();
    assert_eq!(sum, 367662);
}

#[test]
fn correctprobibility() {
    let library = readfile();
    let (degreesmap, degreesvec) = degreetotal(library);
    let (probibilities, probvec) = degreetonode(degreesmap, degreesvec, 0);
    let mut sum: f32 = probvec.iter().sum();
    assert!(sum > 0.99);
}