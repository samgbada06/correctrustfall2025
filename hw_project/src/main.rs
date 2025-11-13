trait ShowInfo {
    fn show_info(&self);
}

struct StudentCore {
    gpa: f32,
    major: String,
}

struct Undergrad {
    core: StudentCore,
}

struct Grad {
    core: StudentCore,
    thesis: String,
}


impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!(
            "Undergrad | Major: {} | GPA: {:.2}",
            self.core.major, self.core.gpa
        );
    }
}


impl ShowInfo for Grad {
    fn show_info(&self) {
        println!(
            "Graduate | Major: {} | GPA: {:.2} | Thesis: {}",
            self.core.major, self.core.gpa, self.thesis
        );
    }
}

struct Enrollment<T: ShowInfo> {
    students: Vec<T>,
}

impl<T: ShowInfo> ShowInfo for Enrollment<T> {
    fn show_info(&self) {
        for student in &self.students {
            student.show_info();
        }
    }
}

fn main() {
    let undergrads = Enrollment {
        students: vec![
            Undergrad {
                core: StudentCore {
                    gpa: 3.7,
                    major: String::from("CS"),
                },
            },
            Undergrad {
                core: StudentCore {
                    gpa: 3.9,
                    major: String::from("Arts"),
                },
            },
        ],
    };

    let grads = Enrollment {
        students: vec![
            Grad {
                core: StudentCore {
                    gpa: 3.8,
                    major: String::from("Data Science"),
                },
                thesis: String::from("Tableau is very good."),
            },
        ],
    };

    println!("Undergrad:");
    undergrads.show_info();

    println!("\nGrad:");
    grads.show_info();
}
