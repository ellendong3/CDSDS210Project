use std::fs;
use std::io;
use std::collections::HashMap;


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
            degreetonode(degreesmap, degreesvec, 0);
            counter += 1;
        } else {
            let number: usize = input.parse().expect("Not a integer number");
            if number >= 1 && number <= 74 {
                let library = readfile();
                let (degreesmap, degreesvec) = degreetotal(library);
                degreetonode(degreesmap, degreesvec, number);
                counter += 1;
            } else {
                println!("Try again. Bad input");
                io::stdin().read_line(&mut input).expect("Failed to read line"); 
            }
        }
    }

}

fn readfile() -> HashMap<u32, Vec<u32>> {

    let mut vec1: Vec<String> = Vec::new();
    let mut vec2: Vec<String> = Vec::new();

    let file = fs::read_to_string("src/email-Enron.txt");

    for i in file.expect("REASON").split("\r\n") {
        let newstring = i.to_string();
        vec1.push(newstring);

    }

    for i in 0..vec1.len() {
        if i > 3 && i != (vec1.len() - 1) {
            vec2.push((*vec1[i]).to_string());

        }
    }

    let mut pairholder: Vec<Vec<u32>> = Vec::new();

    for i in &vec2 {
        let holder = i.split("\t");
        let mut pair: Vec<u32> = Vec::new();

        for n in holder {
            let int = n.trim().parse().expect("REASON");
            pair.push(int);

        }
        pairholder.push(pair);

    }
    
    let mut pairlibrary: HashMap<u32, Vec<u32>> = HashMap::new();

    for n in 0..36692 {
        let mut totalsent: Vec<u32> = Vec::new();
        for i in &pairholder {
            if i[0] == n {
                totalsent.push(i[1]);
            }

        }
        pairlibrary.insert(n, totalsent);

    }
    
    return pairlibrary

}

fn degreetotal(library: HashMap<u32, Vec<u32>>) -> (HashMap<u32, u32>, Vec<u32>) {
    let mut degreelibrary: HashMap<u32, u32> = HashMap::new();
    let mut degreevector: Vec<u32> = Vec::new();

    for i in 0..36692 {
        let node = library.get(&i);
        let degrees = node.unwrap().len();
        degreevector.push(degrees as u32);
        degreelibrary.insert(i, degrees.try_into().unwrap());

    }

    return (degreelibrary, degreevector)

}

fn degreetonode(degreelibrary: HashMap<u32, u32>, vec: Vec<u32>, number: usize) -> (HashMap<u32, f32>, Vec<f32>) {
    let mut finalprob: HashMap<u32, f32> = HashMap::new();
    let mut degreetonodes: HashMap<u32, u32> = HashMap::new();
    let mut probs: Vec<f32> = Vec::new();

    for i in 0..36692 {
        let degrees: u32 = *degreelibrary.get(&i).unwrap() as u32;
        if degreetonodes.get(&degrees) == None {
            degreetonodes.insert(degrees as u32, 1);

        } else {
            let key = degreetonodes.entry(degrees).or_insert(0);
            *key += 1;

        }

    }

    let mut uniquevec: Vec<u32> = vec.clone();
    uniquevec.sort();
    uniquevec.dedup();

    for i in &uniquevec {
        let key = degreetonodes.entry(*i).or_insert(0);
        let prob = (*key as f32) / (36692 as f32) as f32;
        probs.push(prob);
        finalprob.insert(*i, prob);

    }
    
    if number == 0 {
        for i in &uniquevec {
            let key = finalprob.entry(*i).or_insert(0.0);
            println!("{:?} degree(s) has a frequency of {:?}", i, key);

        }
    } else if number > 0 {

        let mut sortedprobs = probs.clone();
        sortedprobs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sortedprobs.reverse();
        sortedprobs.dedup();

        println!("{:?}", sortedprobs.len());

        for i in 0..number {
            let index = sortedprobs[i];

            for n in &uniquevec {
                let key = finalprob.entry(*n).or_insert(0.0);

                if *key == index {
                    println!("{:?}. {:?} degree(s) has a frequency of {:?}", i + 1, n, index);

                }

            }

        }
    }
    
    return (finalprob, probs)

}



