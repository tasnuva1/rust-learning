use std::fs;
use std::path::Path;

// fn main() {
//     let path = Path::new("./.git/objects");
//     for entry in path.read_dir().expect("read_dir call failed") {
//         if let Ok(entry) = entry {
//             println!("{:?}", entry.path());
//         }
//     }
// }

fn main() {
    let content = commit_info(
        &"8732409nhkhakdng".to_owned(),
        &"something this new change".to_owned(),
    );
    fs::write("demo-HEAD", content).unwrap();
    // fs::write("demo-HEAD", "commit something").unwrap();
}

fn commit_info(hash: &String, massage: &String) -> String {
    let mut store = String::new();

    // store.push_str("commit ");
    // store.push_str("\0");
    // store.push_str("tree ");
    store.push_str(&store.len().to_string());
    store.push_str("\0");
    store.push_str(&hash);
    store.push_str(" ");
    store.push_str("author Fahmida Mashura <fahmidamashura@gmail.com> 1243040974 -0700 ");
    store.push_str("committer Fahmida Mashura <fahmidamashura@gmail.com> 1243040974 -0700");
    store.push_str("\n");
    store.push_str(&massage);
    store.push_str("\n");

    store
}
