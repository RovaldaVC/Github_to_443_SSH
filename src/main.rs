use std::process::Command;
use regex::Regex;
use std::sync::LazyLock;

fn get_remote() -> Option<String> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
} 

fn convert_to_443(url:&str) -> Option<String>{
    static RE:LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"github\.com[:/](.+/.+?)(?:\.git)?$").unwrap()
    });
    let caps = RE.captures(url)?;

    let repo = caps.get(1)?.as_str(); 

    Some(format!("ssh://git@ssh.github.com:443/{}.git", repo)) 
} 

fn set_remote(new_url:&str){
    let status = Command::new("git")
        .args(["remote", "set-url", "origin", new_url])
        .status()
        .expect("failed to run git");

    if !status.success(){
        panic!("git remote set-url failed")
    }
}

fn main() {
    let remote = get_remote().expect("Not a git repo or no origin");

    if let Some(new_url) = convert_to_443(&remote){
        println!("Old: {}", remote);
        println!("New: {}", new_url);
        set_remote(&new_url);
        println!("Remote updated.")
    }else {
        println!("Could not parse Github remote");
    }
}