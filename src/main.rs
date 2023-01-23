// Clones all the JSON files from ./custom into ./julia_scrapers/custom, runs the python script ./julia_scrapers/run.py, then copies the output files from ./julia_scrapers/output into ./julia_website/output and starts the ./julia_website using docker compose

use std::process::Command;

fn main() {
    test("Starting the script...");

    let output = Command::new("bash")
        .arg("-c")
        .arg("pwd && mkdir -p ./custom && mkdir -p ./goebel_website/output && rm -rf ./goebel_scraper/custom && mkdir -p ./goebel_scrapers/custom && cp -a ./custom/. ./goebel_scrapers/custom && cd goebel_scrapers && python3 run.py && cd .. && cp -a ./goebel_scrapers/output/. ./goebel_website/output && docker compose up -d")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn test(prt: &str) {
    println!("{}", prt);
}