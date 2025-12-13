use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("r3")
        .about("r3 测试")
        .version("v1.0")
        // .arg(Arg::new("name").long("config").short('c').help("名字"))
        .arg(Arg::new("name").index(1).help("名字"))
        .arg(Arg::new("name1").index(2).help("名字1"))
        .arg(
            Arg::new("name3")
                .index(3)
                .help("名字3")
                .action(ArgAction::Set),
        )
        .get_matches();

    match matches.get_one::<String>("name") {
        Some(x) => {
            println!("name {}", x);
        }
        None => {}
    }
}
