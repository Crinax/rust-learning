# rust-learning

My steps for Rust

I have bad english, please doesn't kick me :D

## Create new project

In first lesson I found out that to create new project with folder and git, I should execute next command:

```sh
cargo new <project-name>
```

But to initialize new project in existing folder, I should execute:

```sh
cargo init
```

## Hello, world

In this lesson I found out that to output text I should use macro `println!()` (not a function)

Now I can output something to console, yahoo! :D

```rust
fn main() {
  println!("Hello, I'm start learning Rust :D")
}
```
