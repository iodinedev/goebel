// Clones all the JSON files from ./custom into ./julia_scrapers/custom, runs the python script ./julia_scrapers/run.py, then copies the output files from ./julia_scrapers/output into ./julia_website/output and starts the ./julia_website using docker compose

use std::process::Command;

fn main() {
    let output = Command::new("bash")
        .arg("-c")
        .arg("pwd && cp -a ./custom/. ./julia_scrapers/custom && cd julia_scrapers && python3 run.py && cd .. && cp -a ./julia_scrapers/output/. ./julia_website/output && cd julia_website && docker compose up -d")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}