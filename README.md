# rust02_workspace_and_testing
**Learning Rust: how to use workspace, binary, library and testing**  
***version: 1.0  date: 2019-05-03 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/rust02_workspace_and_testing)***  

[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Frust02_workspace_and_testing&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

Read the `Last project` first: https://github.com/bestia-dev/rust01_the_beginning  

## Workspace
For Rust Workspace is a group of projects developed together.  
It is easy to make a binary (main.rs, executable) project. But it is not good for testing.  
It is easy to make a library (lib.rs) but you can not run it.  
With a workplace I can split my application into one binary source code and one library source code. So I have the best of both worlds.  
The result is still just one executable file. That is a great thing with Rust.  
## date_diff
I want to make a simple CLI (console line interface) application that calculates date difference.  
I need it to know how many days I am already on vacation.  
I try to use all small caps and underscores for all the file/folder names because of the case sensitivity of Linux file system.  
## GitHub and Git
I pushed the code of this project to GitHub using Git. I believe you are now reading it from GitHub. I suppose you have Git already installed if you learned from my last project https://github.com/bestia-dev/rust01_the_beginning.  
Cloning a GitHub repository to a local folder is super simple.  
In GitHub click on the `Clone or download` big green button. You can there copy the URL to your clipboard.  
Open a Command prompt. Go to the `/rustprojects` folder and type  
`clone ` and space and then paste the clipboard content with `Ctrl + v`  
Yes, the command prompt after years and years of right clicks now supports the ubiquitous `Ctrl + v`. In Linux bash you have still to use right click for this.  
Press `Enter` to run it.  
It will clone the GitHub repository in a new subfolder.  
BTW if you like you can install the Chrome extension "GitHub Dark Theme".  
## Build and run
Go into this subfolder:  
```
cd rust02_workspace_and_testing
```  
Execute the command to build and run with parameters  
```
cargo run -- 20190909 20190808
```  
You can also do it in two separate steps  
```
cargo build
target\debug\date_diff.exe 20190909 20190808
```  
## Source code
To understand the result let's have a look at the source code in VSCode.  
`File - Open folder - c:\Users\Luciano\RustProjects\rust02_workspace_and_testing\`  
You will use your username here.  
You will see that VSCode has a command `Open Workspace`, but I think it is for another type of Workspace, not the Rust Workspace.  
Let's open the file `/date_diff/src/main.rs`  
I verbosely commented the code so it is most instructional.  
Now open the `/date_diff_lib/lib.rs`  
It has a `test` region and only one true public function.  
All is heavy commented.  
  
## Linux
We want the same code to be compiled for Linux.  
Open the `Debian bash` of WSL. You know that from the first tutorial.  
`cd r` then press Tab key and it will suggest `rustprojects` probably.  
`cd rust02` then press Tab key and it will suggest `rust02_workspace_and_testing/` probably.  
It is practical to learn this Tab key suggestions instead of typing long file/folder names.  
```
cargo build  
target/debug/date_diff 20190909 20190808
```  
The result must be the same as in windows.  
  
## Cargo.toml
The file `Cargo.toml` is the configuration file for the Cargo builder tool.  
It is inside every Rust project or workspace.  
If we look inside the first `/Cargo.toml` we will see that the workspace has 2 members.  
The `date_diff` project has its own `Cargo.toml`. It contains package metadata and dependencies.  
The `date_diff_lib` project `Cargo.toml` file contains only package metadata.  
The `Better TOML` extension for VSCode is useful for this syntax in general.  
The VSCode extension `crates` is useful for working with dependencies in `Cargo.toml`.  
Dependencies are by default downloaded from the standard Rust crate repository `crates.io`.  
For local dependencies use the syntax like this `{ path = "../date_diff_lib" }`.  
## Testing
In the workspace folder run  
```
cargo test
```  
It will execute all tests that are included in the source code.  
Find them in `lib.rs` in the module `mod tests`.  
Test are nothing else then normal functions. They are just decorated with  
`#[test]`  
to let the Cargo tool understand they are tests.  
You can also write tests inside function doc comments. This special comments have three slashes and are located before the function declaration. They are used to generate the documentation. If the test passes it can mean, the documentation is up to date.  
https://learning-rust.github.io/docs/a5.comments_and_documenting_the_code.html  
## VSCode
There is a lot of typing in English so I would like to have a spellchecker. I found the VSCode extension `Spell Right`. When editing a text in the right corner down of the status bar there is an `open eye`. Click on it to choose the language.  
## Next projects
https://github.com/bestia-dev/mem1  
  
## References
https://doc.rust-lang.org/stable/book/  
