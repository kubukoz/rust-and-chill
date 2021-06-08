use std::{collections::LinkedList, env::args, fs::remove_file, path::Path};

use git2::{IndexConflict, Repository};

fn main() {
    let repo_path = args()
        .nth(1)
        .expect("The path to the directory wasn't provided!");

    let repo =
        Repository::open(&repo_path).expect(&format!("Couldn't open repository at {}", repo_path));

    let index = repo.index().expect("Couldn't get index");

    index
        .conflicts()
        .unwrap()
        .map(Result::unwrap)
        .for_each(|conflict| {
            let o = &conflict.our;
            // let chuj = &o.map(|p| p.path);

            println!("{:?}", &conflict.their.map(|p| p.path));

            let our = o.expect("Conflicting file was removed on the left-hand side!");

            let file_path = String::from_utf8(our.path).unwrap();

            let should_delete = conflict.their.is_none();

            if should_delete {
                let file_path = Path::new(&file_path);

                println!("Will REMOVE file {:?}", file_path);

                // let path_to_remove = Path::new(&repo_path).join(Path::new(&file_path));
                // remove_file(path_to_remove).unwrap();

                // repo.index().unwrap().add_path(&file_path).unwrap();
                repo.index().unwrap().remove_path(&file_path).unwrap()
                // repo.index()
                //     .unwrap()
                //     .remove_path(Path::new(&(repo_path.to_owned() + "/" + &file_path)))
                //     .unwrap();
                // index.remove_path(Path::new(&file_path));
            } else {
                println!("Will take THEIR version of {}", file_path)
            };
        });
    /*
    repo.statuses(None)
        .expect("Couldn't list file statuses")
        .iter()
        .for_each(|entry| {
            if entry.status().is_conflicted()
                && entry
                    .path()
                    .unwrap_or_else(|| "")
                    .ends_with("EngineMocking.scala")
            {
                println!("{:#?}", entry.status());
                // println!("huh? {}", entry);
                // repo.checkout_index(
                //     Some(&mut index),
                //     Some(
                //         CheckoutBuilder::new()
                //             .path(entry.path().unwrap())
                //             .use_theirs(true)
                //             .allow_conflicts(true),
                //     ),
                // )
                // .expect(&format!(
                //     "Couldn't check in conflicting file {}",
                //     entry.path().expect("<none>")
                // ));
            }
        }); */
}
