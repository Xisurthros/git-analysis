use std::env;
use std::process::Command;


struct Commit {
    hash: String,
    author: String,
    date: String,
    message: String
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

        for commit in &commits {
            println!("Commit: {}\nAuthor: {}\nDate: {}\nMessage: {}\n", 
                commit.hash, commit.author, commit.date, commit.message);
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
            (lines.next(), lines.next(), lines.next(), lines.next()) {

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

