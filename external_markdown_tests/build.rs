extern crate skeptic;


fn list_markdown_files(dir_path: &str) -> Vec<String> {
    use std::fs::read_dir;
    use std::ascii::AsciiExt;

    let mut out = Vec::new();

    let dir = match read_dir(&dir_path) {
        Ok(dir) => dir,
        Err(e) => panic!("Could not read mdbook directory: {:?}", e),
    };

    for entry in dir {
        let e = match entry {
            Ok(e) => e,
            Err(e) => panic!("Could not read a dir entry: {:?}", e),
        };

        let entry_path = e.path();

        match e.metadata() {
            Ok(ref md) if md.is_dir() => {
                out.append(&mut list_markdown_files(entry_path.to_str().unwrap()))
            },
            Ok(ref md) if !md.is_file() => continue,
            Err(e) => panic!("Could not read the metadata of a dir entry: {:?}", e),
            _ => {},
        };

        let extension = match entry_path.extension() {
            Some(extension) => extension.to_str().unwrap(),
            None => continue,
        };

        if extension.eq_ignore_ascii_case("md") {
            out.push(e.path().to_str().unwrap().to_owned());
        }
    }

    out
}

fn list_all_markdown_files() -> Vec<String> {
    // Walking over the book
    let mut book_files = list_markdown_files("../book/src/");
    book_files.push("../README.md".to_owned());
    book_files
}

fn main() {
    skeptic::generate_doc_tests(&list_all_markdown_files());
}
