use clap::CommandFactory;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let cmd = oneseed::cli::Cli::command();

    // generate man page
    let man_dir = out_dir.join("man");
    fs::create_dir_all(&man_dir)?;
    let man = clap_mangen::Man::new(cmd.clone());
    let mut buf = Vec::new();
    man.render(&mut buf)?;
    fs::write(man_dir.join("1seed.1"), &buf)?;
    eprintln!("generated man/1seed.1");

    // generate shell completions
    let comp_dir = out_dir.join("completions");
    fs::create_dir_all(&comp_dir)?;

    for shell in [
        clap_complete::Shell::Bash,
        clap_complete::Shell::Zsh,
        clap_complete::Shell::Fish,
    ] {
        let mut buf = Vec::new();
        clap_complete::generate(shell, &mut cmd.clone(), "1seed", &mut buf);
        let ext = match shell {
            clap_complete::Shell::Bash => "bash",
            clap_complete::Shell::Zsh => "zsh",
            clap_complete::Shell::Fish => "fish",
            _ => unreachable!(),
        };
        fs::write(comp_dir.join(format!("1seed.{ext}")), &buf)?;
        eprintln!("generated completions/1seed.{ext}");
    }

    Ok(())
}
