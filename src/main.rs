//local modules
pub mod displayer;
pub mod handler;
pub mod formatter;
pub mod types;

fn main() {
    // main things to do:
    // 1: make a types module to seperate the code
    // 2: create a proper error system 
    // 3: remove the redundency of some of the code like when displaying the data
    // 4: use more idomatic pratices with the types and code call to be more inline with traditonal
    //    rust
    // 5: 
    handler::run_program();
}
