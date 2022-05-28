/*
RINS by Alexander Abraham,
a.k.a. "The Black Unicorn", a.k.a. "Angeldust Duke".
Licensed under the MIT license.
*/

use cleasy::App;
use std::path::Path;
use colored::Colorize;
use file_serve::Server;
use std::path::PathBuf;

/// Serves the "build" dir
/// on this address: https://localhost:1024.
fn serve_dir(project_path: String) {
    if dir_is(&project_path) {
        let mut path: PathBuf = PathBuf::new();
        path.push(&project_path);
        let server_instance: Server = Server::new(path);
        println!("{}", format!("Serving your site on address:\n{}\nPress Ctrl+C to quit the server.", server_instance.addr()).cyan().to_string());
        server_instance.serve();
    }
    else {
        println!("{}", format!("The directory does not exist!").red().to_string());
    }
}

/// Checks whether a directory exists and returns
/// a boolean to that effect.
fn dir_is(path: &str) -> bool {
    let mut result: bool = false;
    if Path::new(&path).exists() {
        result = true;
    }
    else {}
return result;
}

/// Rins's command-line interface.
fn cli(){
    let mut rins_cli: App = App::new(
        String::from("RINS"),
        String::from("1.0.0"),
        String::from("Alexander Abraham")
    );
    rins_cli.add_arg(
        "sdir".to_string(),
        "Serve a directory.".to_string(),
        "true".to_string()
    );
    if rins_cli.arg_was_used("sdir".to_string()) {
        serve_dir(
            rins_cli.get_arg_data("sdir".to_string())
        );
    }
    else if rins_cli.version_is() {
        println!(
            "{}",
            rins_cli.version().cyan().to_string()
        );
    }
    else if rins_cli.help_is() {
        println!(
            "{}",
            rins_cli.help().cyan().to_string()
        );

    }
    else {
        println!(
            "{}",
            rins_cli.help().red().to_string()
        );
    }
}

/// The main entry point
/// for the Rust compiler.
fn main(){
    cli();
}
