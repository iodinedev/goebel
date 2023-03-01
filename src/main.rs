// Clones all the JSON files from ./custom into ./julia_scrapers/custom, runs the python script ./julia_scrapers/run.py, then copies the output files from ./julia_scrapers/output into ./julia_website/output and starts the ./julia_website using docker compose

extern crate cronjob;

use std::process::Command;
use cronjob::CronJob;

fn main() {
    // Run the script once
    run("");

    // Run the script every 60 minutes
    let mut cron = CronJob::new("Update Content", run);

    // crontab: 0 * * * *

    // Every hour
    cron.seconds("0");
    cron.minutes("0");
    cron.hours("*");

    cron.start_job();

}

fn run(_: &str) {
    test("Starting the script...");

    let output = Command::new("bash")
        .arg("-c")
        .arg("
            mkdir -p ./custom &&
            mkdir -p ./goebel_website/output &&
            rm -rf ./goebel_scraper/custom &&
            mkdir -p ./goebel_scrapers/custom &&
            cp -a ./custom/. ./goebel_scrapers/custom &&
            cd goebel_scrapers &&
            source venv/bin/activate &&
            pip install -r requirements.txt &&
            python3 run.py &&
            deactivate &&
            cd .. &&
            cp -a ./goebel_scrapers/output/. ./goebel_website/output &&
            docker compose up -d
        ")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    // If there is an error, save it to a file because it is too long to print
    if !output.stderr.is_empty() {
        std::fs::write("error.txt", output.stderr).expect("Unable to write file");
    }
}

fn test(prt: &str) {
    println!("{}", prt);
}