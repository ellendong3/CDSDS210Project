use std::fs;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;


fn main() {
    //println!("Hello, world!");
    let library = readfile();
    let (degreesmap, degreesvec) = degreetotal(library);
    let degreenodes = degreetonode(degreesmap);
    println!("{:?}", degreenodes.get(&70));

}

fn readfile() -> HashMap<u32, Vec<u32>> {

    let mut cat_ing = HashMap::<String, Vec<String>>::new();
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

    //println!("{:?}", pairholder[0][0]);
    
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

fn degreetonode(degreelibrary: HashMap<u32, u32>) -> HashMap<u32, u32> {
    let mut degreetonodes: HashMap<u32, u32> = HashMap::new();
    for i in 0..36692 {
        let degrees: u32 = *degreelibrary.get(&i).unwrap() as u32;
        if degreetonodes.get(&degrees) == None {
            degreetonodes.insert(degrees as u32, 1);
        } else {
            let key = degreetonodes.entry(degrees).or_insert(0);
            *key += 1;
        }
    }

    return degreetonodes
}

    /*

            for n in holder {
            let pair: Vec<u32> = Vec::new();
            let int = n.trim().parse().expect("REASON");
            pair.push(int);
        }

            for n in i.split("\t") {
            let int: u32 = n.trim().parse().expect("REASON");
        }

    let mut iter = 0;
    let mut titles: Vec<String> = Vec::new();
    let mut ingr: Vec<String> = Vec::new();


    for i in vec1 {
        for n in i.split(":") {
            if iter % 2 == 0 {
                titles.push(n.to_string());
            } else {
                ingr.push(n.to_string())
            }
            iter += 1
        }
    }
    
    let mut cleaningredients: Vec<Vec<String>> = Vec::new();

    for i in ingr {
        let mut onerecipe: Vec<String> = Vec::new();
        for n in i.trim().split(", ") {
            onerecipe.push(n.to_string());
        }
        cleaningredients.push(onerecipe);
        
    }
    //println!("{:?}", cleaningredients);

    for i in 0..(titles.len() - 1) {
        //println!("{:?}", i);
        cat_ing.insert(titles[i].clone(), cleaningredients[i].clone());
        
    }
    return cat_ing

    */



