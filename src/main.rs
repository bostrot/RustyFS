use clap::Parser;
use handlebars::Handlebars;
use regex::Regex;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::Arc,
};
use webserver::ThreadPool;

// Import logging.rs file
mod logging;
use logging::Logging;

// Import macros.rs file
#[macro_use]
mod macros;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path mandatory
    #[arg(short = 'd', long, required = true)]
    path: String,

    /// Number of threads
    /// Default: 4
    #[arg(short, long, default_value = "4")]
    threads: Option<usize>,

    /// Port
    /// Default: 7878
    #[arg(short, long, default_value = "7878")]
    port: Option<String>,

    /// Host
    /// Default: 127.0.0.1
    #[arg(short = 'i', long, default_value = "127.0.0.1")]
    host: Option<String>,

    /// Verbose
    /// Default: false
    /// Note: This will print the line number and file path
    #[arg(short, long)]
    verbose: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let path = Arc::new(PathBuf::from(&args.path));

    // Check path
    if !path.exists() {
        panic!("Path does not exist");
    }

    // Set verbosity
    Logging::set_verbose(args.verbose.unwrap_or(false));

    // Listen for connections
    let listener = TcpListener::bind(format!(
        "{}:{}",
        args.host.unwrap_or("127.0.0.1".to_string()),
        args.port.as_ref().unwrap_or(&"7878".to_string())
    ).as_str())
    .unwrap();
    Logging::info(f!("Listening on port {}", args.port.unwrap_or("7878".to_string())));

    let pool = ThreadPool::new(args.threads.unwrap_or(4));
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };
        let aras = Arc::clone(&path);
        pool.execute(|| {
            handle_connection(stream, aras);
        });
    }
}

#[derive(Serialize)]
struct CustomFile {
    path: String,
    name: String,
}

fn build_page(path: Arc<PathBuf>, ospath: Arc<PathBuf>) -> (&'static str, String) {
    // Setup handlebars
    let mut handlebars = Handlebars::new();
    match handlebars.register_template_string("index", include_str!("../assets/index.html")) {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to register template: {}", e);
        }
    }
    let mut data = BTreeMap::new();

    // Get files in directory
    let mut files: Vec<CustomFile> = Vec::new();
    let paths = match fs::read_dir(path.as_os_str()) {
        Ok(paths) => paths,
        Err(e) => {
            error!("Failed to read directory: {}", e);
            return ("HTTP/1.1 500 Internal Server Error", "".to_string());
        }
    };

    for path in paths {
        let name = &path
            .as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .to_string();
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        let ospath = ospath.to_str().unwrap();
        let path = path.replace(ospath, "");

        // Get file name
        files.push(CustomFile {
            path: path.to_string(),
            name: name.to_string(),
        });
    }

    // Add to handlebars data
    let handlebar_render = match handlebars.render("index", &data) {
        Ok(render) => render,
        Err(e) => {
            error!("Failed to render template: {}", e);
            return ("HTTP/1.1 500 Internal Server Error", "".to_string());
        }
    };
    data.insert("files".to_string(), files);
    (
        "HTTP/1.1 200 OK",
        handlebar_render
    )
}

fn handle_connection<'a>(mut stream: TcpStream, ospath: Arc<PathBuf>) {
    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next() {
        Some(line) => match line {
            Ok(line) => line,
            Err(e) => {
                error!("Failed to read request line: {}", e);
                return;
            }
        },
        None => {
            error!("Failed to read request line");
            return;
        }
    };

    let (status_line, contents) = if request_line == "GET / HTTP/1.1" {
        build_page(Arc::clone(&ospath), ospath)
    } else {
        // Check if the request is for a file
        let re = Regex::new(r"GET /(.*) HTTP/1.1").unwrap();
        let cap = re.captures(&request_line).unwrap();
        let file = cap.get(1).unwrap().as_str();
        let file_path = Path::new(&file);
        let file_path = ospath.join(file_path);

        // Unescape the path
        let file_path = file_path.to_str().unwrap();
        let file_path = urlencoding::decode(file_path).unwrap();
        let file_path = Path::new(file_path.as_ref());

        // File exists
        if file_path.exists() {
            // Is directory
            if file_path.is_dir() {
                let path = Arc::new(file_path.to_path_buf());
                build_page(path, ospath)
            } else {
                // Read non UTF-8 file
                let length = file_path.metadata().unwrap().len();

                let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n");

                stream.write_all(response.as_bytes()).unwrap();
                stream.write_all(&fs::read(file_path).unwrap()).unwrap();

                ("HTTP/1.1 200 OK", "contents".to_string())
            }
        } else {
            (
                "HTTP/1.1 404 NOT FOUND",
                include_str!("../assets/404.html").to_string(),
            )
        }
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
