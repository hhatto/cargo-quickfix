use argparse::{ArgumentParser, StoreOption, StoreTrue};
use serde_json::Value;
use std::env;
use std::io::{stderr, stdout};
use std::process::{exit, Command};

static ERRMSG: &str = "invalid json message";

fn main() {
    let mut target_filename: Option<String> = None;
    let mut verbose = false;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut verbose).add_option(&["--verbose"], StoreTrue, "Verbose output");
        ap.refer(&mut target_filename)
            .add_argument("target-filename", StoreOption, "Target filename");

        ap.parse(env::args().skip(1).collect(), &mut stdout(), &mut stderr())
            .map_err(exit)
            .ok();
    }

    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("check");
    cargo_command.arg("--message-format").arg("json");

    let output = cargo_command.output().expect("fail");
    let status = output.status;
    if !status.success() && verbose {
        println!("{:?}", output);
    }

    let stdout = output.stdout;
    let lines = std::str::from_utf8(&stdout).unwrap().lines();
    for line in lines {
        let data: Value = serde_json::from_str(line).unwrap();
        let obj = data.as_object().unwrap();
        let m = obj.get("message");
        if m.is_none() {
            continue;
        }
        let message = m.unwrap().as_object().expect(ERRMSG);

        if let Some(span) = message.get("spans").unwrap().as_array().unwrap().first() {
            let span = span.as_object().expect(ERRMSG);
            let filename = span.get("file_name").expect(ERRMSG);
            if let Some(ref target_filename) = target_filename {
                if target_filename != filename {
                    continue;
                }
            }
            let level = message.get("level").expect(ERRMSG);
            let msg = message.get("message").expect(ERRMSG);
            let line_number = span.get("line_end").expect(ERRMSG);
            let column_number = span.get("column_start").expect(ERRMSG);

            println!(
                "{}:{}:{}: {}: {}",
                filename.as_str().unwrap(),
                line_number,
                column_number,
                level.as_str().unwrap(),
                msg.as_str().unwrap()
            );
        }
    }

    std::process::exit(status.code().unwrap_or(1));
}
