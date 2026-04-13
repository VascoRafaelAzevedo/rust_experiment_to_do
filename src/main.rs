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
    let mut tasks_list: Vec<Task> = vec![];

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input!");
    user_input = user_input.trim().to_owned();

    match user_input.as_str() {
        "add" => add_task(&mut tasks_list),
        "rem" => remove_task(&mut tasks_list),
        "list" => list_tasks(&tasks_list),
        "edit" => edit_task(&mut tasks_list),
        _ => println!("Command Not Found!"),
    }
}

fn edit_task(tasks_list: &[Task]) {
    let edit_id = chose_task(tasks_list);

    clear_terminal();
    // show banner
    let banner = make_banner("Edit Task".to_owned(), Style::NeonCyber);

    match banner {
        Ok(banner) => println!("{}", banner),
        Err(e) => println!("Error in Banner {}", e),
    }
    //end show banner
    println!("{}", tasks_list[edit_id].title);
    println!("-----------------------------");
    println!("Description: {}", tasks_list[edit_id].description);
    println!("Priority: {}", tasks_list[edit_id].priority);
    println!("-----------------------------");
    println!("Insert what to do: (h for help)");

    loop {

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input!");
        match user_input.trim() {
            "priority" => change_priority(),
            "description" => change_description(),
            "back" => return,
            "h" => edit_help()
    }
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn chose_task(tasks_list: &[Task]) -> usize {
    clear_terminal();
    // show banner
    let banner = make_banner("Chose Task".to_owned(), Style::NeonCyber);

    match banner {
        Ok(banner) => println!("{}", banner),
        Err(e) => println!("Error in Banner {}", e),
    }
    //end show banner

    for task in tasks_list {
        println!("{} - {}", task.id, task.title);
    }

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input!");
        match user_input.trim().parse::<i32>() {
            Ok(id) => return id as usize,
            Err(_) => println!("Invalid Number!"),
        }
    }
}
fn list_tasks(tasks_list: &[Task]) {}

fn remove_task(tasks_list: &[Task]) {
    todo!()
}

fn add_task(tasks_list: &[Task]) {
    todo!()
}

fn main() {
    let _ = welcome_banner();

    app_loop();
}
