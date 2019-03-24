use super::config;
use std::process::Command;
use time::PreciseTime;

pub struct CommandExecutor {
    pub execution_times: Vec<ExecutionRecord>,
    display_output: bool,
}

#[derive(Debug)]
pub struct ExecutionRecord {
    pub name: String,
    pub execution_time: i64,
}

impl CommandExecutor {
    pub fn new(display_output: bool) -> CommandExecutor {
        CommandExecutor {
            execution_times: vec![],
            display_output,
        }
    }

    pub fn execute_task_groups(&mut self, task_groups: Vec<config::TaskGroup>) {
        for task_group in task_groups.iter() {
            self.execute_task_group(task_group)
        }
    }

    fn execute_task_group(&mut self, task_group: &config::TaskGroup) {
        self.execute_task_helper_command(&task_group.initialization_command);

        for task in task_group.tasks.iter() {
            self.execute_task_helper_command(&task.setup_command);
            self.execute_task_command(&task_group.name, &task);
            self.execute_task_helper_command(&task.tear_down_command);
        }

        self.execute_task_helper_command(&task_group.cleanup_command);
    }

    fn execute_task_helper_command(&self, helper_command: &Vec<String>) {
        if helper_command.len() > 0 && helper_command[0] != "" {
            self.execute_command(&helper_command[0], &helper_command[1..]);
        }
    }

    fn execute_command(&self, command: &str, arguments: &[String]) {
        let mut process = Command::new(command);
        process.args(arguments);
        let output = process.output().expect("Failed to execute command");

        let result = String::from_utf8(output.stdout);
        match result {
            Ok(value) => {
                if self.display_output {
                    println!("{}", value);
                }
            },
            Err(message) => panic!("{:?}", message),
        }
    }

    fn execute_task_command(&mut self, group_name: &str, task: &config::Task) {
        for _i in 0..task.repetition_count {
            let command = &task.command;
            let result = self.measure_execution_time(&command[0], &command[1..]);
            if let Some(execution_time) = result {
                let name = self.determine_command_name(group_name, command[0].clone());
                self.store_execution_time(&name, execution_time);
            }
        }
    }

    fn determine_command_name(&self, group_name: &str, command: String) -> String {
        if group_name == "" {
            return command
        }

        format!("{}/{}", group_name, command)
    }

    fn measure_execution_time(&self, command: &str, arguments: &[String]) -> Option<i64> {
        let start = PreciseTime::now();
        self.execute_command(command, arguments);
        let end = PreciseTime::now();

        start.to(end).num_nanoseconds()
    }

    fn store_execution_time(&mut self, name: &str, execution_time: i64) {
        self.execution_times.push(ExecutionRecord {
            name: name.to_string(),
            execution_time: execution_time,
        });
    }
}
