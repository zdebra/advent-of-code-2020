use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world from day 7!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut rule_set = HashMap::new();
    let mut vv_rule_set = HashMap::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let top_bag_color = get_base_color_code(line.clone());
        let rule = rule_set
            .entry(top_bag_color.clone())
            .or_insert(HashMap::new());

        let content = get_content(line.clone());
        content.iter().for_each(|(bag, count)| {
            rule.insert(bag.clone(), *count);
            let vice_versa_rule = vv_rule_set.entry(bag.clone()).or_insert(HashMap::new());
            vice_versa_rule.insert(top_bag_color.clone(), *count);
        });
    }

    // part 1

    let starting_point = vv_rule_set
        .get("shiny gold")
        .expect("expected shiny gold to be found in vv rule set");
    let mut queue = vec![starting_point];
    let mut valid_bags = vec![];
    loop {
        if let Some(item) = queue.pop() {
            for (bag, count) in item.iter() {
                if valid_bags.iter().position(|x| x == bag).is_some() {
                    continue;
                }
                valid_bags.push(bag.clone());
                if let Some(r) = vv_rule_set.get(bag) {
                    queue.push(r);
                }
            }
        } else {
            break;
        }
    }

    valid_bags.iter().for_each(|x| print!("{}, ", x));
    println!("in total: {}", valid_bags.len());

    // part 2

    let starting_point = rule_set
        .get("shiny gold")
        .expect("expected shiny gold to be found in ule set");
    let mut queue = vec![(starting_point, 1)];

    let mut total = 0;

    loop {
        if let Some((item, multiply_with)) = queue.pop() {
            for (bag, count) in item.iter() {
                let multiplier = count * multiply_with;
                total += multiplier;

                // add new edges to the queue
                if let Some(r) = rule_set.get(bag) {
                    queue.push((r, multiplier));
                }
            }
        } else {
            break;
        }
    }

    println!("totalec: {}", total);
}

fn get_base_color_code(line: String) -> String {
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut color_partials = Vec::new();
    for word in words {
        if word != "bags" {
            color_partials.push(word);
        } else {
            break;
        }
    }
    color_partials.join(" ")
}

fn get_content(line: String) -> HashMap<String, usize> {
    let splits: Vec<&str> = line.split("contain").collect();
    assert!(splits.len() == 2);
    let contents = splits[1]
        .trim()
        .strip_suffix(".")
        .expect("expected dot at the end of line");
    let mut output = HashMap::new();
    for content in contents.split(",") {
        let mut count = 0;
        let mut color_code_partials = Vec::new();
        if content.trim() == "no other bags" {
            continue;
        }
        for (i, content_word) in content.trim().split(" ").enumerate() {
            if i == 0 {
                count = content_word
                    .parse::<usize>()
                    .expect("expected first word of content split to be a number");
                continue;
            }
            match content_word {
                "bag" | "bags" => break,
                _ => color_code_partials.push(content_word),
            }
        }
        output.insert(color_code_partials.join(" "), count);
    }
    output
}
