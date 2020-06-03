#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Level {
    Show(String),
    Seq {
        show: String,
        seq: String,
    },
    Shot {
        show: String,
        seq: String,
        shot: String,
    },
}

pub fn main() {
    println!("ENUMS");
    part1();
    if_let_bindings();
    let lvl = Level::Shot {
        show: "DEV01".into(),
        seq: "RD".into(),
        shot: "0001".into(),
    };

    mods();

    match_level(&lvl);

    match_show(&lvl);

    match_guards();

    printit(&lvl);

    get_show(&lvl);
}

// Basic Level
fn part1() {
    let show = Level::Show("DEV01".into());
    let lvl = Level::Shot {
        show: "DEV01".into(),
        seq: "RD".into(),
        shot: "0001".into(),
    };
    println!("{:?}\n{:?}", &show, &lvl);
}

fn if_let_bindings() {
    println!("\nif_let_bindings");
    let show = Level::Show("DEV01".into());
    // notice that this is an assignment, not an equality test
    if let Level::Show(show) = show {
        println!("I extracted a show: {}", show);
    } else {
        println!("I didnt find a show");
    }
}

fn match_level(level: &Level) {
    println!("\nmatch_level");
    match &level {
        // branches
        Level::Show(show) => println!("show: {}", show),
        // elision
        Level::Seq { show, seq } => println!("show {}, seq: {}", show, seq),
        Level::Shot { show, seq, shot } => println!(
            "the show: {} has a sequence: {} with a shot: {}",
            show, seq, shot
        ),
    }
}

fn match_show(level: &Level) {
    println!("\nmatch_show");
    match &level {
        // branches
        Level::Show(show) => println!("show: {}", show),
        // elision
        Level::Seq { show, .. } => println!("show {}", show),
        Level::Shot { show, .. } => println!("show {}", show),
    }
}

fn mods() {
    let mut lvl = Level::Show("dev01".into());

    if let Level::Show(ref mut show) = lvl {
        *show = show.to_uppercase();
    }
    println!("new lvl is {:?}", lvl);
}

fn mods2() {
    let mut level = Level::Shot {
        show: "dev01".into(),
        seq: "rd".into(),
        shot: "0001".into(),
    };

    match level {
        // branches
        Level::Show(ref mut show) => {
            println!("show: {}", show);
            *show = show.to_uppercase();
        }
        // elision
        Level::Seq { show, seq } => println!("show {}, seq: {}", show, seq),
        Level::Shot {
            ref mut show,
            ref mut seq,
            ref mut shot,
        } => {
            *show = show.to_uppercase();
            *seq = seq.to_uppercase();
            println!(
                "the show: {} has a sequence: {} with a shot: {}",
                show, seq, shot
            );
        }
    }
}

fn match_guards() {
    let id = 3;
    match id {
        1 | 2 | 3 => println!("small"),
        4 | 5 | 6 => println!("med"),
        _ => println!("large"),
    }
}

impl Level {
    pub fn printme(&self) {
        match self {
            Self::Show(show) => println!("show: {}", show),
            Self::Seq { show, seq } => println!("show {}, seq: {}", show, seq),
            Self::Shot { show, seq, shot } => println!(
                "the show: {} has a sequence: {} with a shot: {}",
                show, seq, shot
            ),
        }
    }

    pub fn show(&self) -> String {
        match self {
            Self::Show(show) => show.clone(),
            Self::Seq { show, .. } => show.clone(),
            Self::Shot { show, .. } => show.clone(),
        }
    }
}

fn printit(level: &Level) {
    level.printme();
}

fn get_show(lvl: &Level) {
    let show = lvl.show();
    println!("I own a show: {}", show);
}
