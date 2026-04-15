use core::fmt;
use std::io;
use tui_banner::{Banner, Style};
#[derive(Debug)]
enum Priority {
    P0,
    P1,
    P2,
    P3,
    NP,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let priority_str = match self {
            Priority::P0 => "P0",
            Priority::P1 => "P1",
            Priority::P2 => "P2",
            Priority::P3 => "P3",
            Priority::NP => "No priority",
        };
        write!(f, "{priority_str}")
    }
}

#[derive(Debug)]
struct Task {
    id: i32,
    title: String,
    priority: Priority,
    description: String,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::new(),
            priority: Priority::NP,
            description: String::from("No description"),
        }
    }
}

impl Task {
    fn set_priority(&mut self, new_p: Priority) {
        self.priority = new_p;
    }

    fn set_description(&mut self, new_d: String) {
        self.description = new_d;
    }
}

fn welcome_banner() -> Result<(), tui_banner::BannerError> {
    println!();
    let banner = Banner::new("Rust DO")?
        .style(Style::NeonCyber)
        .align(tui_banner::Align::Center)
        .render();
    println!("{}", banner);
    Ok(())
}

fn make_banner(title: String, style: tui_banner::Style) -> Result<String, tui_banner::BannerError> {
    let banner = Banner::new(title)?
        .style(style)
        .align(tui_banner::Align::Left)
        .render();
    Ok(banner)
}

fn app_loop() {
    let mut tasks: Vec<Task> = vec![];

    println!("Please insert command:");
    print!(">");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("fail to read input");

    let input_arguments: Vec<String> = user_input.split_whitespace().map(String::from).collect();
    match input_arguments[0].as_str() {
        "add" => add_task(&tasks, &input_arguments[1..]),
        _ => {
            println!(
                "Error, no command {} found, type help for the command list.",
                input_arguments[0]
            )
        }
    }
}

fn edit_task(tasks_list: &Task) {}
fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn list_tasks(tasks_list: &Task) {
    todo!()
}

fn remove_task(tasks_list: &Task) {
    todo!()
}

fn add_task(tasks_list: &mut Vec<Task>, new_input: &[String]) {
    let new_Task = Task {
        id: tasks_list.len() as i32,
        title: new_input.join(" "),
        ..Default::default()
    };
    tasks_list.push(new_Task);
}

fn main() {
    let _ = welcome_banner();

    app_loop();
}
