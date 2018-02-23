use std::path::Path;

pub fn add_prefix_to_file_name(file_name: &str, prefix: &str) -> String {
    let path = Path::new(file_name);
    let file_name = path.file_name().unwrap();
    let dir = path.parent().unwrap();
    if dir.to_str().unwrap() == "" {
        return format!("{}{}", prefix, file_name.to_str().unwrap());
    }
    format!("{}/{}{}",
            dir.to_str().unwrap(),
            prefix,
            file_name.to_str().unwrap())
}

#[test]
fn shoud_add_prefix_with_dir() {
    let filename = add_prefix_to_file_name(&"foo/bar/filename.png", &"sample_");
    assert_eq!(filename, "foo/bar/sample_filename.png");
}

#[test]
fn shoud_add_prefix_without_dir() {
    let filename = add_prefix_to_file_name(&"filename.png", &"sample_");
    assert_eq!(filename, "sample_filename.png");
}