use clap::{
    App,
    Arg,
    SubCommand,
    crate_version,
    crate_authors,
    crate_description,
};

struct Command <'a, 'b> {
    name: &'static str,
    alias: &'static str,
    desc: &'static str,

    args: Option<Vec<Arg<'a, 'b>>>,
}

fn generate_command <'a, 'b> (attrs: Command<'a, 'b>) -> App<'a, 'b> {
    let initialized_command = SubCommand::with_name(attrs.name).alias(attrs.alias).about(attrs.desc);
    let command: App<'a, 'b>;

    if attrs.args != None {
        command = initialized_command.args(&attrs.args.unwrap());
    }

    command
}

pub fn init_app () {
    let create_command: App = generate_command(Command {
        name: "create",
        alias: "c",
        desc: "Create a base command file in the commands directory",
        args: None,
    });

    let app = App::new("alf")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(create_command);

    check_matches(app);
}

fn check_matches(app: App) {
    let matches = app.get_matches();

    if None == matches.subcommand_name() {
        println!();
        println!("Hello !");
        println!("It looks like the command you entered does not exist,");
        println!("Please have a look to --help")
    }
}