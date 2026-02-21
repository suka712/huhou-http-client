use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: hz <METHOD> <URL>");
        std::process::exit(1);
    }

    let method = &args[1];
    let url = &args[2];

    let client = reqwest::blocking::Client::builder()
        .user_agent("hh/0.1.0")
        .build()
        .unwrap();
    let response = client.request(method.parse().unwrap(), url).send().unwrap();

    println!("{:?} {}", response.version(), response.status());

    for (name, value) in response.headers() {
        println!("{}: {}", name, value.to_str().unwrap());
    }

    println!();
    println!("{}", response.text().unwrap());
}
