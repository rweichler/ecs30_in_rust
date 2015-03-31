

pub struct Parser {
    courses: Vec<Course>
}

pub struct Course {
    crn: i32,
    subject: String,
    number: String
}

use std::fmt;

impl Course {
    fn new(crn: i32, subject: String, number: String) -> Course {
        Course {
            crn: crn,
            subject: subject,
            number: number
        }
    }
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.crn, self.subject, self.number)
    }
}

impl Parser {
    pub fn new(contents:&str) -> Parser{
        let mut courses = vec![];

        let mut new_line = true;
        let mut on_crn = false;
        let mut on_subject = false;
        let mut on_number = false;
        let mut crn = 0;
        let mut subject = String::new();
        let mut number = String::new();
        for letter in contents.chars() {
            if letter == '\n' {
                new_line = true;
            }
            else if new_line {
                new_line = false;
                if letter >= '0' && letter <= '9' {
                    crn = letter as i32 - '0' as i32;
                    on_crn = true;
                }
            }
            else if on_crn {
                if letter >= '0' && letter <= '9' {
                    crn = crn * 10 + letter as i32 - '0' as i32;
                }
                else
                {
                    on_crn = false;
                    if crn < 10000 || crn > 99999 {
                        crn = 0;
                    }
                    else
                    {
                        on_subject = true;
                        subject = String::new();
                        number = String::new();
                    }
                }
            }
            else if on_subject {
                if letter == '\t' || letter == '^' {
                    if subject.len() == 3 {
                        on_subject = false;
                        on_number = true;
                    }
                }
                else
                {
                    subject.push(letter);
                }
            }
            else if on_number {
                if letter == '\t' && number.len() > 1{
                    on_number = false;
                    let obj = Course::new(crn , subject.clone(), number.clone());
                    courses.push(obj);
                }
                else
                {
                    number.push(letter);
                }
            }
        }
        return Parser { courses: courses };
    }

    pub fn from_crn(self: &Parser, crn: i32)
    {
        let mut found = false;
        for course in &self.courses {
            if course.crn == crn {
                println!("{} {}", course.subject, course.number);
                found = true;
            }
        }
        if !found {
            println!("lol not found");
        }
    }

    pub fn from_subject(self: &Parser, subject: String)
    {
        let mut found = false;
        for course in &self.courses {
            if course.subject == subject {
                println!("{}", course);
                found = true;
            }
        }
        if !found {
            println!("lol not found");
        }
    }
}
