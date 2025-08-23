## README

to run your executable like this 
``` 
$ program <args>
``` 
instead of this 
```
$ cargo run -- <args>
```
first run the following two commands in this folder:
```
ln -s target/debug/main <program_name>
export PATH="./:$PATH"
```
