use clap::{Arg, ArgMatches, Command};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use std::process;
use std::str::FromStr;

fn main() {
    let matches = Command::new("Dominion")
        .version("0.1.0")
        .author("Michael Gilliland <github.com/mjgpy3>")
        .about("Generate dominion kingdoms")
        .arg(
            Arg::new("include-expansions")
                .short('e')
                .long("include-expansions")
                .takes_value(true)
                .value_name("EXPANSION")
                .multiple_values(true)
                .help_heading("LIMITING")
                .help("Expansions from which to take cards"),
        )
        .arg(
            Arg::new("project-count")
                .short('p')
                .long("project-count")
                .takes_value(true)
                .value_name("NUMBER")
                .help_heading("LIMITING")
                .help("Include a number of projects")
                .possible_values(["0", "1", "2"]),
        )
        .arg(
            Arg::new("bane-count")
                .long("bane-count")
                .takes_value(true)
                .value_name("NUMBER")
                .help_heading("LIMITING")
                .help("Include a number of bane expansion cards (experimental/custom)")
                .possible_values(["0", "1", "2", "3"]),
        )
        .arg(
            Arg::new("ban-cards")
                .short('b')
                .long("ban-cards")
                .takes_value(true)
                .value_name("CARD")
                .multiple_values(true)
                .help_heading("LIMITING")
                .help("Ensure these cards are not included"),
        )
        .arg(
            Arg::new("include-cards")
                .short('c')
                .long("include-cards")
                .takes_value(true)
                .value_name("CARD")
                .multiple_values(true)
                .help_heading("LIMITING")
                .help("Ensure these cards are included"),
        )
        .arg(
            Arg::new("output-code")
                .long("code")
                .help_heading("OUTPUT")
                .help("Output history code with the setup (largely) filled out"),
        )
        .arg(
            Arg::new("output-raw")
                .long("raw")
                .help_heading("OUTPUT")
                .help("Output raw setup structure (from rust debug dump)"),
        )
        .arg(
            Arg::new("output-pretty")
                .long("pretty")
                .help_heading("OUTPUT")
                .help("Prettify output to make physical setup easier"),
        )
        .arg(
            Arg::new("output-hists")
                .long("hists")
                .help_heading("OUTPUT")
                .help("Write histograms"),
        )
        .arg(
            Arg::new("output-name")
                .long("name")
                .help_heading("OUTPUT")
                .help("Generate a kingdom name"),
        )
        .get_matches();

    let config = dominion::SetupConfig {
        include_expansions: optional_set(&matches, "include-expansions"),
        ban_cards: optional_set(&matches, "ban-cards"),
        include_cards: optional_set(&matches, "include-cards"),
        project_count: matches
            .value_of("project-count")
            .map(|_| matches.value_of_t_or_exit("project-count")),
        bane_count: matches
            .value_of("bane-count")
            .map(|_| matches.value_of_t_or_exit("bane-count")),
    };

    let setup = dominion::gen_setup(config);

    match setup {
        Ok(setup) => {
            let name = if matches.is_present("output-name") {
                let name = dominion::game_name::random(&setup);
                println!("== {} ==", name);
                name
            } else {
                "Game".to_string()
            };

            if matches.is_present("output-pretty") {
                println!("------------------ SETUP ------------------");
                println!("");
                println!("{}", dominion::pretty::pretty(&setup));
                println!("");
            }

            if matches.is_present("output-raw") {
                println!("------------------ RAW ------------------");
                println!("");
                println!("{:?}", setup);
                println!("");
            }

            if matches.is_present("output-code") {
                println!("------------------ CODE ------------------");
                println!("");
                println!("{}", dominion::pretty::code(name, &setup));
                println!("");
            }

            if matches.is_present("output-hists") {
                println!("------------------ HISTS ------------------");
                println!("");
                println!("{}", dominion::pretty::hists(&setup));
                println!("");
            }
        }
        Err(err) => {
            eprintln!(
                "Error generating kingdom!\n\n{}",
                dominion::pretty::gen_error(err)
            );
            process::exit(1);
        }
    }
}

fn optional_set<R: FromStr + Clone + Hash + Eq>(
    matches: &ArgMatches,
    key: &str,
) -> Option<HashSet<R>>
where
    <R as FromStr>::Err: Display,
{
    matches
        .value_of(key)
        .map(|_| matches.values_of_t_or_exit(key).iter().cloned().collect())
}
