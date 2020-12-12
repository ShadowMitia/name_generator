use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use unicode_segmentation::UnicodeSegmentation;

// TODO: adding testing!
// TODO: add more stuff and do more stuff!

struct MarkovChain {
    starts: HashMap<String, f32>,
    nexts: HashMap<String, HashMap<String, f32>>,
}

impl MarkovChain {
    // TODO: change function, and don't take step as float
    // TODO: (not urgent), make this class work for mulitple types?
    fn from(data: &[String], step: f32) -> Self {
        let mut starts: HashMap<String, f32> = HashMap::new();
        let mut nexts: HashMap<String, HashMap<String, f32>> = HashMap::new();

        for s in data {
            let graphemes = s.graphemes(true).collect::<Vec<&str>>();
            starts
                .entry(
                    graphemes
                        .iter()
                        .take(step as usize)
                        .map(|&s| s.to_string())
                        .collect::<String>(),
                )
                .and_modify(|t| *t += 1.0)
                .or_insert(1.0);

            let sub = graphemes
                .iter()
                .take(step as usize)
                .map(|&s| s.to_string())
                .collect::<String>();

            let next = graphemes
                .iter()
                .skip(1)
                .take(step as usize)
                .map(|&s| s.to_string())
                .collect::<String>();

            nexts
                .entry(sub)
                .and_modify(|t| {
                    *t.entry(next.clone()).or_insert(1.0) += 1.0;
                })
                .or_default()
                .insert(next, 1.0);

            let mut s = String::from(s);
            s.push('.');
            let graphemes = s.graphemes(true).collect::<Vec<&str>>();

            let n = graphemes.len() as i32 - step as i32;

            for i in 1..=(n) {
                let sub = graphemes
                    .iter()
                    .skip(i as usize)
                    .take(step as usize)
                    .map(|&s| s.to_string())
                    .collect::<String>();

                let next = graphemes
                    .iter()
                    .skip(i as usize + 1)
                    .take(step as usize)
                    .map(|&s| s.to_string())
                    .collect::<String>();

                nexts
                    .entry(sub)
                    .and_modify(|t| {
                        *t.entry(next.clone()).or_insert(1.0) += 1.0;
                    })
                    .or_default()
                    .insert(next, 1.0);
            }
        }

        Self { starts, nexts }
    }

    fn select_random_item(items: &HashMap<String, f32>) -> String {
        let mut rnd: f32 = (rand::random::<f32>() * items.values().sum::<f32>()) as f32;

        for (k, _) in items.iter() {
            rnd -= *items.get(k).unwrap() as f32;
            if rnd < 0.0 {
                return k.to_owned();
            }
        }

        String::new()
    }

    fn generate(&mut self) -> String {
        let mut start = MarkovChain::select_random_item(&self.starts);
        let mut result = String::from(&start);

        while let Some(item) = self.nexts.get(&start) {
            start = MarkovChain::select_random_item(&item);

            if start.ends_with('.') {
                break;
            }
            result.push_str(start.graphemes(true).nth_back(0).unwrap());
        }

        result
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut company_names = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("company_names.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(name) = line {
                company_names.push(name.to_lowercase());
            }
        }
    }

    let names: Vec<String> = vec![
        "Dimitri",
        "Jean-Philippe",
        "Alfred",
        "Bruce",
        "Sébastien",
        "Frédéric",
        "Mona",
        "Molière",
        "Grégoire",
        "Louis-Philippe",
        "Helena",
        "Alain",
        "Etienne",
        "Alexandre",
        "Robert",
        "Amélie",
        "Aurélia",
        "Gabrielle",
        "Aude",
        "Agathe",
        "Bernard",
        "Bob",
        "Bertrand",
    ]
    .iter()
    .map(|s| s.to_lowercase())
    .collect();

    let jap_names: Vec<String> = vec![
        "あきら",
        "はやと",
        "ひろこ",
        "ほたる",
        "けいすけ",
        "こたろう",
        "まさる",
        "みのる",
        "まな",
        "あまや",
        "べにこ",
        "だいき",
        "だいすけ",
        "はると",
        "めぐみん",
        "つぐに",
        "はじめ",
    ]
    .iter()
    .map(|s| s.to_lowercase())
    .collect();

    let mut gen = MarkovChain::from(&names, 3.0);
    let mut jap_gen = MarkovChain::from(&jap_names, 3.0);

    let mut test = Vec::new();
    let mut jap_test = Vec::new();

    let mut companies = Vec::new();
    let mut companies_gen = MarkovChain::from(&company_names, 4.0);

    while companies.len() < 20 {
        // TODO: add more rules to filter badly looking names

        let name = companies_gen.generate();
        if company_names
            .iter()
            .any(|s: &String| s.to_lowercase() == name)
        {
            continue;
        }
        companies.push(name);
    }

    while test.len() < 20 {
        // TODO: add more rules to filter badly looking names

        let name = gen.generate();
        if names.iter().any(|s: &String| s.to_lowercase() == name) {
            continue;
        }
        test.push(name);
    }

    while jap_test.len() < 20 {
        // TODO: add more rules to filter badly looking names
        // TODO: needs more checking that generation works
        let name = jap_gen.generate();
        if jap_names.iter().any(|s: &String| s.to_lowercase() == name) {
            continue;
        }
        jap_test.push(name);
    }

    for company in companies {
        println!("{}", company);
    }

    println!("=================");

    for name in test {
        println!("{}", name);
    }

    println!("=================");

    for test in jap_test {
        println!("{}", test);
    }
}
