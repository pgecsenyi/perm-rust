#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub task_groups: Vec<TaskGroup>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskGroup {
    pub cleanup_command: Vec<String>,
    pub initialization_command: Vec<String>,
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub command: Vec<String>,
    pub repetition_count: u32,
    pub setup_command: Vec<String>,
    pub tear_down_command: Vec<String>,
}
