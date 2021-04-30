use std::fmt;
struct Structure(i32);
impl fmt::Display for Structure{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Structure({})", self.0)
    }

}

fn main() {
    println!("Hello, world!");
    let cc = Structure(9898);
    println!("cc is : {}",cc);
}
