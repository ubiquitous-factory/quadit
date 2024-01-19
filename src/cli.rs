use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
pub struct QuaditCli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
    //ExampleDerive(Vers),
}

// #[derive(clap::Args)]
// #[command(version, about, long_about = None)]
// pub struct Version {
//     #[arg(long)]
//     pub manifest_path: Option<std::path::PathBuf>,
// }
