use std::fmt;
struct Structure(i32);
struct List(Vec<i32>);

impl fmt::Display for Structure{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Structure({})", self.0)
    }

}

impl fmt::Display for List {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f,"[")?;
        for (count,v ) in vec.iter().enumerate() {
            if count != 0 { write!(f,", ")?;}
            write!(f,"{0}:{1}",count,v)?;
        }
        write!(f,"]")
    }
}

fn main() {
    println!("Hello, world!");
    let cc = Structure(9898);
    println!("cc is : {}",cc);
    let cv = List(vec![12,32,4433,4545,908348]);
    println!("cv is : {}",cv);
}
