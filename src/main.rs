use clap::Parser;
use std::{fs, env, path};
use dotenv::from_path;
use chrono::Local;
use subprocess::{Popen, PopenConfig};
use shellexpand::tilde;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the topic. E.g.: "meetings", "meetings.tasks"
    #[arg(short, long)]
    topic: String,

    /// Name of the file
    #[arg(short, long, default_value_t = String::from("papers"))]
    name: String
}

fn main() {
    let expanded = tilde(&"~/.config/papers/.env").to_string();
    let env_path = path::Path::new(&expanded);
    from_path(env_path).expect("Set the environment variables in ~/.config/papers/.env.");

    let args = Args::parse();
    let root_dir = env::var("ROOT_DIR").expect("ROOT_DIR must be set.");
    let extension = env::var("EXTENSION").expect("EXTENSION must be set.");
    let editor_command = env::var("EDITOR_COMMAND").expect("EDITOR_COMMAND must be set.");
    let topic_path = path::Path::new(&root_dir).join(args.topic.replace(".", "/"));

    let file_name =
        Local::now().format("%Y%m%d%H%M%S").to_string()
        + "_"
        + &args.name
        + "."
        + &extension;

    let file_path = topic_path.join(file_name);

    fs::create_dir_all(topic_path).expect("Couldn't create directories.");

    Popen::create(&[
        editor_command,
        file_path.to_str().unwrap().to_string()
    ], PopenConfig {..Default::default()})
    .expect("Couldn't open papers.");
}
