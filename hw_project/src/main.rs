struct Student {
    name: String,
    major: String,
}

impl Student{
    fn new(n:String, m: String) -> Student {
        Self {
            name: n,
            major: m,
        }
    }

    fn set_major(&mut self, n_major: String){
        self.major = n_major;
    }

    fn get_major(&self) -> &String{
        return &self.major
    }

    
}

fn main() {
    let mut Durk = Student::new("Durk".to_string(), "CS".to_string());
    
    println!("{}", Durk.get_major());
    Durk.set_major("Business".to_string());
    println!("{}", Durk.get_major());


}
