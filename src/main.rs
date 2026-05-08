use std::process::Command;
use regex::Regex;
use std::sync::LazyLock; // Lazy means Compile once, use as much as you want.

fn get_remote() -> Option<String> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .ok()?; // turns Result<Output, Error> into: Option<T>
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
} // get_remote gives us the url of our remote repository.

fn convert_to_443(url:&str) -> Option<String>{
    static RE:LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"github\.com[:/](.+/.+?)(?:\.git)?$").unwrap()
    });  // here we create a structure, (.+/.+) can be replaced with whatever we give to it. next part.
    let caps = RE.captures(url)?; // we gave (.+/.+) our url!

    let repo = caps.get(1)?.as_str(); //get(1) will returm (.+/.+) which means (user/repo)

    Some(format!("ssh://git@ssh.github.com:443/{}.git", repo)) // variable:repo has user/repo inside it!
} // we will get the url based on 443 at the end.

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

//static means Global use, wherever.

/* 
can't use static RE:Regex
static needs a compile-time type validation.
but Regex is not a compile time fn, it's runtime fn!
key solution is: when we say Lazy<Regex> it will become like a struct.

Lazy<Regex> is like:
struct Lazy{
value: Maybe<Regex>
}
*/