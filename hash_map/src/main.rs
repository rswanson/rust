
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 11);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    println!("the score for {team_name} was {score}");

    for (k, v) in &scores {
        println!("{k} : {v}");
    }
    
    scores.insert(team_name, 12); // insert will overwrite if the key exists

    // this won't work anymore - insert takes ownership of strings 
    // println!("{team_name}"); 

    println!("{:#?}", scores);

    // entry doesn't 
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:#?}", scores);

}
