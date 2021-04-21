use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"),  10);
    scores.insert(String::from("yellow"), 40);
    let teams = vec![String::from("yearn"),String::from("purple")];
    let initial_scores = vec![10,50];
    let mut zscores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Hello, world!");
    println!("yearn is : {:?}",zscores.get(&String::from("yearn")));
    println!("{:?}",zscores);
    for (key,value) in &zscores{
        println!("{}: {}",key,value);
    }
    zscores.entry(&String::from("yan")).or_insert(&1008);
    let text = "hello world wonderful world";
    let mut smap = HashMap::new();
    for word in text.split_whitespace(){
        let count = smap.entry(word).or_insert(0);
        *count = *count + 1;
    }
    println!("{:?}",smap);
}
