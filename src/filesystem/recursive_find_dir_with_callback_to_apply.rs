use std::fs::{self, DirEntry};

fn main() {

    let dir = "/path/to/dir";

    visit_dirs(dir, &|e: &DirEntry| {println!("{}", e.path().to_str().unwrap());})


}


// one possible implementation of walking a directory only visiting files
fn visit_dirs<'a>(dir: &'a Path,  cb: &'a dyn Fn(&DirEntry)) -> io::Result<()> {

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path,  cb)?;
            } else {
                cb(&entry);
            }
        }
    }

    Ok(())

}