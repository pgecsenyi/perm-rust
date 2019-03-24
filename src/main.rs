extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;

mod command_executor;
mod config;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;

use command_executor::CommandExecutor;
use command_executor::ExecutionRecord;
use config::Config;

#[derive(StructOpt)]
struct Cli {
    config_path: String,
    output_path: String,
    display_output: bool,
    generate_sample_config: bool,
}

fn main() {
    let args = Cli::from_args();
    if args.generate_sample_config {
        generate_sample_config(&args.config_path);
        println!("Configuration generates successfully to {}.", &args.config_path);
        return
    }

    let configuration_result = read_config(&args.config_path);
    let configuration = match configuration_result {
        Ok(content) => parse_config(&content),
        Err(error) => panic!("Could not read file. {:?}", error),
    };

    match configuration {
        Ok(value) => measure_execution_time(value.task_groups, &args.output_path, args.display_output),
        Err(error) => panic!("Could not read file. {:?}", error),
    };
}

fn generate_sample_config(output_file: &str) {
    let sample_config = create_sample_config();
    let config_json = serde_json::to_string_pretty(&sample_config)
        .expect("Unable to generate sample configuration.");

    let mut file = File::create(output_file)
        .expect("Unable to open output file.");
    file.write_all(config_json.as_bytes())
        .expect("Unable to write data.");
}

fn create_sample_config() -> config::Config {
    let sample_task = config::Task {
        command: vec![String::from("echo"), String::from("Hello World!")],
        repetition_count: 1,
        setup_command: vec![],
        tear_down_command: vec![]
    };
    let sample_task_group = config::TaskGroup {
        cleanup_command: vec![],
        initialization_command: vec![String::from("clear")],
        name: String::from("Greeting"),
        tasks: vec![sample_task]
    };

    config::Config {
        task_groups: vec![sample_task_group]
    }
}

fn read_config(filename: &str) -> Result<String, io::Error> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();

    buf_reader.read_to_string(&mut content)?;

    Ok(content)
}

fn parse_config(data: &str) -> Result<Config, io::Error> {
    let config: Config = serde_json::from_str(data)?;

    Ok(config)
}

fn measure_execution_time(task_groups: Vec<config::TaskGroup>, output_path: &str, display_output: bool) {
    let mut command_executor = CommandExecutor::new(display_output);
    command_executor.execute_task_groups(task_groups);

    let result_export = export_result(&command_executor.execution_times, output_path);
    match result_export {
        Ok(_value) => println!("Result exported to {}.", output_path),
        Err(error) => panic!("Could not export result. {:?}", error),
    }
}

fn export_result(
    execution_times: &Vec<ExecutionRecord>,
    output_path: &str,
) -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_path(output_path)?;

    for record in execution_times {
        wtr.write_record(&[record.name.clone(), record.execution_time.to_string()])?;
    }

    wtr.flush()?;

    Ok(())
}
