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
Yes, the command prompt lately after years and years of right clicks now supports the ubiquitous `Ctrl + v`. In Linux bash you have still to use right click for this.  
Press `Enter` to run it.  
It will clone the GitHub repository in a new subfolder.  
## Build and run
Go into this subfolder:  
`cd rust02_workspace_and_testing`  
Execute the command to build and run with parameters  
`cargo run -- 20190909 20190808`  
You can also do it in two separate steps..
`cargo build`  
then 
`cd target`  
`cd debug`  
`date_diff.exe 20190909 20190808`  
## Source code
To understand the result let's have a look at the source code in VSCode.  
`File - Open folder - c:\Users\Luciano\RustProjects\rust02_workspace_and_testing\`
You will have your username here.  
You will see that VSCode has `Open Workspace`, but I think it is another type of Workspace, not the Rust Workspace.  
Let's open `/date_diff/src/main.rs`  
I verbosely commented the code so it is most instructional.  
Now open the `/date_diff_lib/lib.rs`  
It has a `test` region and only one true public function.  
All is heavy commented.  
  
## Linux
We want the same code to be compiled to Linux.  
Open the Debian bash. 
`cd r` then press Tab key and it will suggest `rustprojects` probably.  
`cd rust02` then press Tab key and it will suggest `rust02_workspace_and_testing/` probably.  
`cargo build`  
`cd target`  
`cd debug`  
and now run the executable binary  
`./date_diff 20190909 20190808`  
The result must be the same as in windows.  
  
## Referenced
https://doc.rust-lang.org/stable/book/  




