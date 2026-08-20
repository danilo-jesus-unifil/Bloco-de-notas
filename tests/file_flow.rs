use std::fs;

#[allow(dead_code)]
#[path = "../src/document/mod.rs"]
mod document;
#[allow(dead_code)]
#[path = "../src/error/mod.rs"]
mod error;
#[path = "../src/file_io/mod.rs"]
mod file_io;

#[test]
fn saves_and_reopens_utf8_text_with_bom() {
    let directory =
        std::env::temp_dir().join(format!("bloco-de-notas-test-{}", std::process::id()));
    fs::create_dir_all(&directory).expect("test directory should be created");
    let path = directory.join("documento.txt");

    file_io::save(&path, "linha 1\nlinha 2", true, document::LineEnding::CrLf)
        .expect("document should be saved");
    let loaded = file_io::load(&path).expect("document should be reopened");

    assert!(loaded.utf8_bom);
    assert_eq!(loaded.line_ending, document::LineEnding::CrLf);
    assert_eq!(loaded.text, "linha 1\nlinha 2");

    let _ = fs::remove_dir_all(directory);
}
