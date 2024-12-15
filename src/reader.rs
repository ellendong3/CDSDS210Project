use std::fs;
use std::collections::HashMap;

pub fn readfile() -> HashMap<u32, Vec<u32>> {

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