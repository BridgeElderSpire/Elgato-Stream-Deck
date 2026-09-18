use std::collections::HashMap;

struct Student {
    name: String,
    age: u32,
    score: f64,
}

struct Classroom {
    students: HashMap<String, Student>,
}

impl Classroom {
    fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, name: &str, age: u32, score: f64) {
        self.students.insert(
            name.to_string(),
            Student {
                name: name.to_string(),
                age,
                score,
            },
        );
    }

    fn average_score(&self) -> f64 {
        if self.students.is_empty() {
            return 0.0;
        }

        self.students
            .values()
            .map(|student| student.score)
            .sum::<f64>()
            / self.students.len() as f64
    }

    fn highest_score(&self) -> Option<&Student> {
        self.students.values().max_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap()
        })
    }

    fn print_report(&self) {
        println!("Classroom Report");
        println!("================");

        let mut students: Vec<&Student> = self.students.values().collect();
        students.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        for student in students {
            println!(
                "{} | Age: {} | Score: {:.1}",
                student.name,
                student.age,
                student.score
            );
        }

        println!("================");
        println!("Students: {}", self.students.len());
        println!("Average Score: {:.2}", self.average_score());

        if let Some(student) = self.highest_score() {
            println!("Top Student: {}", student.name);
            println!("Top Score: {:.1}", student.score);
        }
    }
}

fn main() {
    let mut classroom = Classroom::new();

    classroom.add_student("Alice", 20, 94.5);
    classroom.add_student("Brian", 21, 87.0);
    classroom.add_student("Clara", 19, 98.0);
    classroom.add_student("David", 22, 91.5);

    classroom.print_report();
}