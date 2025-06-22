use printpdf::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io;
use std::path::Path;

fn calculate_average(total_marks: f32, num_subjects: u32) -> f32 {
    total_marks / num_subjects as f32
}

fn assign_grade(avg: f32) -> &'static str {
    if avg >= 90.0 {
        "A"
    } else if avg >= 75.0 {
        "B"
    } else if avg >= 60.0 {
        "C"
    } else {
        "D"
    }
}

fn generate_pdf(name: &str, total: f32, subjects: u32, avg: f32, grade: &str) {
    use printpdf::*;

    let (doc, page1, layer1) = {
        let mut doc = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
        let page1 = doc.get_page(1);
        let layer1 = page1.get_layer(1);
        (doc, page1, layer1)
    };

    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .expect("Failed to load font");

    let content = format!(
        "Student Report Card\n\nName: {}\nTotal Marks: {}\nNumber of Subjects: {}\nAverage: {:.2}\nGrade: {}",
        name, total, subjects, avg, grade
    );

    let text_lines: Vec<&str> = content.split('\n').collect();
    let mut y_position = Mm(250.0);

    for line in text_lines {
        layer1.use_text(line, 14.0, Mm(20.0), y_position, &font);
        y_position -= Mm(10.0);
    }

    let file = std::fs::File::create("report_card.pdf").unwrap();
    doc.save(&mut BufWriter::new(file)).unwrap();
}

fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut num_subjects = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();
    println!("Enter total marks:");
    io::stdin().read_line(&mut total_marks).unwrap();
    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects).unwrap();

    let name = name.trim();
    let total_marks: f32 = total_marks.trim().parse().unwrap();
    let num_subjects: u32 = num_subjects.trim().parse().unwrap();

    let avg = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(avg);

    println!("\n--- Report Card ---");
    println!("Name: {}", name);
    println!("Total Marks: {}", total_marks);
    println!("Number of Subjects: {}", num_subjects);
    println!("Average: {:.2}", avg);
    println!("Grade: {}", grade);

    generate_pdf(name, total_marks, num_subjects, avg, grade);
    println!("\n📄 PDF report card generated as `report_card.pdf`.");
}
