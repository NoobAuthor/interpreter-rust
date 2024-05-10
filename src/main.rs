mod lexer;
mod repl;
mod token;

fn main() {
    println!("Welcome to the Monkey programming language!");
    println!("Type in commands");
    repl::start();
}
