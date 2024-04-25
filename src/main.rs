use std::fs;
use std::io::{self, Read};
use diff::lines;

fn read_file_to_string(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn compare_documents(doc1: &str, doc2: &str) {
    let changes = lines(&doc1, &doc2);
    for change in changes {
        match change {
            diff::Result::Left(l) => println!("- {}", l),
            diff::Result::Both(_, r) => println!("  {}", r),
            diff::Result::Right(r) => println!("+ {}", r),
        }
    }
}

fn main() -> io::Result<()> {
    let doc1_path = "path_to_document_version_1.txt";
    let doc2_path = "path_to_document_version_2.txt";

    let doc1 = read_file_to_string(doc1_path)?;
    let doc2 = read_file_to_string(doc2_path)?;

    println!("Comparing documents...\n");
    compare_documents(&doc1, &doc2);

    Ok(())
}
