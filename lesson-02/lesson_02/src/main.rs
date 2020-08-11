#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl Level {
    pub fn show(&self) {
        match self {
            Level::Show(show) => {
                let myshow = show.to_uppercase();
                println!("{}", myshow)
            }
            Level::Seq { show, .. } => println!("{}", show),
            Level::Shot { show, .. } => println!("{}", show),
        }
    }
}

fn main() {
    let mut lev = Level::Show("dev01".into());
    let mut lev2 = Level::Seq {
        show: "dev01".into(),
        seq: "rd".into(),
    };
    //assert_eq!(Level::Show, Level::Show);
    //assert!(Level::Show < Level::Seq);
    println!("Hello, world!, {:?}", lev2);

    if let Level::Show(ref mut show) = lev2 {
        *show = show.to_uppercase();
        println!("{}", &show);
    } else {
        println!("no show");
    }
    println!("\n find that show");
    match &lev {
        Level::Show(show) => {
            let myshow = show.to_uppercase();
            println!("{}", myshow)
        }
        Level::Seq { show, .. } => println!("{}", show),
        Level::Shot { show, .. } => println!("{}", show),
    }

    let num = 2;

    match num {
        1 | 2 => println!("small"),
        3 | 4 | 5 => println!("med"),
        _ => println!("large"),
    }

    lev2.show();

    let mut myshow = Some(lev);
    println!("{:?}", myshow);
    let foo = myshow.map(|x| Level::Show("foo".to_string()));
    if foo.is_some() {
        println!("yes");
    }
    println!("{:?}", foo);
    let foo: Option<String> = None;
    println!("{:?}", foo.unwrap());
}
