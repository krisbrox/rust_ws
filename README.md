## README

### Getting Started

You first need to get your rust environment up and running.

1. Compiler: the rust toolchain is delivered through `rustup`, which you can install from brew with `brew install rustup`
2. IDE: Intellij IDEA Ultimate works fine, as does RustRover (also by Jetbrains). Visual studio should have one or more rust plugins probably giving the same or
   similar functionality.
3. Most development-related stuff like building, testing etc. is done using `cargo`, e.g. ``cargo build`` to compile your program into an executable.
4. See``cargo --help`` for more
5. It is highly recommended to configure your IDE to run `rustfmt`, the standard rust formatter, on save.

### Tasks

The idea with this workshop is to give you an idea of what you can build to learn rust.
There are of course a million ideas for this to be found from a simple google search, but it can be hard to choose something
for your first project. In the `/src`-folder you'll find the skeleton of a small CLI calculator application, which you can
repurpose for e.g. a `grep` replacement. If you prefer then you may of course implement an infix variant of the expression parsing
and evaluation you can find in `/src/lib/example`, or add implementation of more interesting math than the already implemented
"integer addition".

Lastly, it is _highly recommended_ to work together in pairs or trios, and please do not hesitate to ask the organizers to explain
anything you find yourself struggling to get a good understanding of.

Small tip: to run your executable like this

``` 
$ <program_name> <args>
``` 

instead of this

```
$ cargo run -- <args>
```

you can run the following two commands in this folder:

```
ln -s target/debug/main <program_name>
export PATH="./:$PATH"
```
