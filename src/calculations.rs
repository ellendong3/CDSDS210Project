use std::collections::HashMap;

pub fn degreetotal(library: HashMap<u32, Vec<u32>>) -> (HashMap<u32, u32>, Vec<u32>) {
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

pub fn degreetonode(degreelibrary: HashMap<u32, u32>, vec: Vec<u32>, number: usize) -> (HashMap<u32, f32>, Vec<f32>) {
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