# git443
A simple Rust CLI that converts a GitHub repository remote to use SSH over port 443.
This is useful in restricted networks where the default SSH port (22) is blocked but HTTPS (443) is allowed.
GitHub supports SSH over 443 via `ssh.github.com`.

Example conversion:
git@github.com:user/repo.git → ssh://git@ssh.github.com:443/user/repo.git

## Features
- Detects current `origin` remote
- Converts GitHub URLs automatically
- Updates the remote to use port 443

## Installation
Clone the repository and build:
cargo build --release

The binary will be located at:
target/release/git443

Optional: install globally
cargo install --path .

## Usage
Run the tool inside any Git repository:
git443

Example output:
Old: git@github.com:user/repo.git  
New: ssh://git@ssh.github.com:443/user/repo.git  
Remote updated.

## Requirements
- Rust
- Git installed
- Repository must use GitHub

## License
MIT