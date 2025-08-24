## README

### Getting Started

You first need to get your rust environment up and running.

1. Compiler: the rust toolchain is delivered through `rustup`, which you can install from brew with `brew install rustup`
2. IDE: Intellij IDEA Ultimate works fine, as does RustRover (also by Jetbrains). Visual studio should have one or more rust plugins probably giving the same or
   similar functionality.

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
