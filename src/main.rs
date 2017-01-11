extern crate rustc_serialize;

use std::process::Command;
use rustc_serialize::json::Json;

const ERRMSG: &'static str = "invalid json message";

fn main() {
    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("build");
    cargo_command.arg("--message-format").arg("json");

    let target_filename = std::env::args().nth(1).unwrap();

    let output = cargo_command.output().expect("fail");
    if !output.status.success() {
        println!("{:?}", output);
    }

    let stdout = output.stdout;
    let lines = std::str::from_utf8(&stdout).unwrap().lines();
    for line in lines {
        let data = Json::from_str(line).unwrap();
        let obj = data.as_object().unwrap();
        let message = obj.get("message").expect(ERRMSG).as_object().expect(ERRMSG);

        let level = message.get("level").expect(ERRMSG).as_string().expect(ERRMSG);
        let msg = message.get("message").expect(ERRMSG).as_string().expect(ERRMSG);
        let span = message.get("spans").unwrap().as_array().unwrap()[0].as_object().expect(ERRMSG);
        let filename = span.get("file_name").expect(ERRMSG).as_string().expect(ERRMSG);
        if filename != target_filename {
            continue;
        }
        let line_number = span.get("line_end").expect(ERRMSG);
        let column_number = span.get("column_start").expect(ERRMSG);

        println!("{}", format!("{}:{}:{}: {}: {}", filename, line_number, column_number, level, msg));
    }

    std::process::exit(1);
}
