use std::{env, fs::{self, read_to_string}, path::PathBuf};

use comrak::{markdown_to_html, Options};
use maud::{html, PreEscaped, DOCTYPE};
use tiny_http::{Response, Server};

pub fn main() {
    let address = "127.0.0.1:8080";
    println!("http://{}", address);

    let cwd: PathBuf = env::current_dir().expect("Could not get cwd").join("src");
    println!("{}", cwd.display());

    let src: PathBuf = cwd.clone().join("src");
    let docs: PathBuf = cwd.clone().join("docs");

    _ = fs::remove_dir_all(&docs);
    _ = fs::create_dir(&docs);

    build_docs(&src, &docs);

    let server = Server::http(address).unwrap();
    // let mut count = 0;

    loop {
        let request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => { println!("error: {}", e); continue; }
        };

        // let split_url = request.url().split('/').collect::<Vec<&str>>()[1..].to_vec();
        println!("{}", request.url().trim_prefix());
        _ = request.respond(Response::from_file(request.u));
    }
}

fn build_docs(src: &PathBuf, docs: &PathBuf) {
    if let Ok(files) = fs::read_dir(src) {
        println!("{:?}", files);

        for file in files {
            
            if let Ok(asd) = file {
                if let Ok(file_type) = asd.file_type() {
                    let src_new = src.clone().join(asd.file_name());
                    let docs_new = docs.clone().join(asd.file_name());

                    if (file_type.is_dir()) {
                        fs::create_dir(&docs_new).expect("Could not create folder");
                        build_docs(
                            &src_new,
                            &docs_new
                        );  
                    } else {
                        if let Ok(contents) = fs::read_to_string(&src_new) {
                            let html = comrak::markdown_to_html(contents.as_str(), &Options::default());
                            fs::write(docs_new, html).expect("Could not write to file");
                        }
                    }
                    println!("{:?}, {:?}", asd, file_type.is_dir());
                }
            }
        }
    }
}