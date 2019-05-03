Things are changing fast. This is the situation on 2019-05-03.  
# rust02_workspace_and_testing
Learning Rust: how to use workspace, binary, library and testing.  
Read the first part first: https://github.com/LucianoBestia/rust01_the_beginning  
## Workspace
For Rust Workspace is a group of projects developed together.  
It is easy to make a binary (main.rs, executable) project. But it is not good for testing.  
It is easy to make a library (lib.rs) but you can not run it.  
With a workplace I can split my application into one binary source code and one library source code. So I have the best of both worlds.  
The result is still just one executable file. That is a great thing with Rust.  
## date_diff
I want to make a simple CLI (console line interface) application that calculates date difference. I need it to know how many days I am on vacation.  
I try to use all small caps and underscores for all the file/folder names because of the case sensitivity of Linux file system.  
## GitHub and Git
I pushed the code of this project to GitHub using Git. I believe you are now reading it from GitHub. I suppose you have Git already installed if you learned from my last project `rust01_the_beginning`.  
Cloning a GitHub repository to a local folder is super simple.  
In GitHub click on the `Clone or download` big green button. You can there copy the URL to your clipboard.  
Open a Command prompt. Go to the `/rustprojects` folder and type  
`clone `  and then paste the clipboard content with `Ctrl + v`  
Yes, the command prompt lately after years and years of right clicks now supports the ubiquitous `Ctrl + v`.  
Press `Enter` to run it.  
It will clone the GitHub repository in a new subfolder.  
## Build and run
Go into this subfolder:  
`cd rust02_workspace_and_testing`  
Execute the command to build and run with parameters  
`cargo run -- 20190909 20190808`  

