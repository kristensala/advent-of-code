use core::fmt;
use anyhow::Result;

enum Command {
    Ls,
    Cd
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Ls => write!(f, "ls"),
            Command::Cd => write!(f, "cd")
        }
    }
}

struct File {
    path: String,
    size: i64
}

impl File {
    fn new(file_path: String, file_size: i64) -> File {
        File { path: file_path, size: file_size }
    }
}

struct Directory {
    path: String,
    files: Vec<File>,
    dirs: Vec<String>
}

impl Directory {
    fn new(new_path: String, new_files: Vec<File>, directories: Vec<String>) -> Directory {
        return Directory {
            path: new_path,
            files: new_files,
            dirs: directories
        };
    }
}

struct Terminal {
    current_command: Option<Command>,
    working_dir: String
}

impl Terminal {
    fn new() -> Terminal {
        return Terminal {
            current_command: None,
            working_dir: String::from("")
        };
    }

    fn parse_line(line: &str) {
        // todo
    }

    fn change_working_dir(&mut self, dir: &str) {
        let current_dir = self.working_dir.clone();
        if dir == ".." {
            //todo; take the last part from the working dir
        } else {
            self.working_dir = current_dir + format!("/{}", dir).as_str();
        }
    }

    fn parse_user_command(&mut self, row: &str) {
        let split: Vec<&str> = row.split(" ").collect(); // if user command expect 2 if cd then 3
        let command = split[1];

        let cd_command = Command::Cd.to_string().as_str();
        let ls_command = Command::Ls.to_string().as_str();

        match command {
            cd_command => {
                self.current_command = Some(Command::Cd);
                self.change_working_dir(split[2])
            },
            ls_command => {
                self.current_command = Some(Command::Ls);
            }
        };
    }

    fn list_directory(&self) {
         
    }

    fn is_user_command(row: &str) -> bool {
        return row.starts_with("$");
    }

    fn is_directory(row: &str) -> bool {
        return row.starts_with("dir");
    }

    fn get_file(&self, row: &str) -> Option<File> {
        let split: Vec<&str> = row.split(" ").collect();
        let is_number = split[0].parse::<i64>();

        return match is_number {
            Ok(file_size) => Some(File::new(self.working_dir.clone(), file_size)),
            Err(_)=> None
        };
    }
}


fn main() {
    let mut terminal = Terminal::new();

    let lines: Vec<&str> = include_str!("../../input/input7.test")
        .lines()
        .collect();
    
    //play
    for line in lines {
        if Terminal::is_user_command(line) { // either ls or cd
            // todo: parse usercommand 
        } else { // this should be right after ls command

        }
    }

}

