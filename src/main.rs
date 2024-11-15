use std::env;
use std::process::Command;
use std::io::{self, Write};

struct Commit {
    hash: String,
    author: String,
    date: String,
    message: String,
}

fn main() {
    let repo_path = env::var("HOME").expect("Could not find home directory") + "/programming/IT-Website";
    let output = Command::new("git")
        .arg("-C")
        .arg(&repo_path)
        .arg("log")
        .arg("--pretty=format:%H%n%an%n%ad%n%s%n---")
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        let commit_data = String::from_utf8_lossy(&output.stdout);
        let commits = parse_commits(&commit_data);

        let user_input = get_user_input();

        match user_input.as_str() {
            "commits" => {
                for commit in &commits {
                    println!(
                        "Commit: {}\nAuthor: {}\nDate: {}\nMessage: {}\n",
                        commit.hash, commit.author, commit.date, commit.message
                    );
                }
            }
            "author count" => {
                let author_name = get_author_name();
                let count = count_author_commits(&commits, &author_name);
                println!("Number of commits by {}: {}", author_name, count);
            }
            _ => println!("Invalid command!"),
        }
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn parse_commits(output: &str) -> Vec<Commit> {
    let mut commits = Vec::new();

    for commit_text in output.split("\n---\n") {
        let mut lines = commit_text.lines();

        if let (Some(hash), Some(author), Some(date), Some(message)) =
            (lines.next(), lines.next(), lines.next(), lines.next())
        {
            commits.push(Commit {
                hash: hash.to_string(),
                author: author.to_string(),
                date: date.to_string(),
                message: message.to_string(),
            });
        }
    }

    commits
}

fn get_user_input() -> String {
    print!("Enter command (commits/author count): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn get_author_name() -> String {
    print!("Enter author name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut author_name = String::new();
    io::stdin()
        .read_line(&mut author_name)
        .expect("Failed to read line");

    author_name.trim().to_string()
}

fn count_author_commits(commits: &[Commit], author_name: &str) -> usize {
    commits.iter()
        .filter(|commit| commit.author == author_name)
        .count()
}
