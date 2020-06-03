# Product Types, Sum Types, AKA Tagged Unions, or we've been cheated all these years

One of Rust's great features not really seen outside of functional languages (unless you count union. and you absolutey shouldnt) is the Sum Type. To review watered down set theory as it applies to types, *product types* are those types that combine other types using 'and', and *sum types* combine other types using 'or'.

Product types are meat and potatoes types that you are already used to. Lists, tuples, hashmaps, etc. Yawn.

Sum types, however are a more exotic, and a lot more powerful. In Rust, the type which falls under this category is *enum*. Rust's enum is unike the enum that you are probably used to in c or c++, and have complained about the lack fo in python. Because Rust's enum is a full blown sum type. We will go over all of its great features, but first, lets step back and look at how you create an enum. You do so using the *enum* keyword:

```rust
pub enum Level {
    Show,
    Seq,
    Shot
}
```
Instantiating a variant is rather simple as well:
```rust
let level = Level::Shot;
assert_eq!(level,Level::Shot);
```
Ok, well there is nothing particuary exotic there, eh? Kind of run of the mill. But wait, there's more. Because each variant is not limited to a simple name. Each variant may be a struct or a tuplestruct as well.

```rust
pub enum Level {
    Show(String),
    Seq{show: String, seq: String},
    Shot{pub show: String, pub seq: String, pub shot: String}
}

let show = Level::Show(stringify!(DEV01));
let lvl = Level::Shot{show: stringify!(DEV01), seq: stringify!(RD), shot: stringify(0001)};
```

So the first problem as you might guess is accessing said data. While you could do what I have above and make the fields public, that is not how its done. Instead, we rely on destructuring. 

## if let
The first form that we will look at is the `if let` form, and it looks like this:
```rust
if let Level::Show(show) = level {
    println!("show: {}", show);
} else {
    println!("not a show");
}
```

### Destructuring

This is destructuring, not unlike python's tuple destructuring, except that this can be arbitrarily complex. Now you might be wondering about ownership, at this point. The result of a destructure is dependent on the type of the data (owned vs non) and the use of modifiers.

You may add a couple of them - `ref` and `mut`. 
```rust

let mut lvl = Level::Show("dev01".into());

if let Level::Show(ref mut show) = lvl {
    *show = show.to_uppercase();
}
println!("the value of lvl is now: {:?}", lvl);
```

Destructuring allows you to extract owned content from containers, as well as reference components.

## match

The most powerful tool for destructuring uses the `match` keyword. Match is most similar to `switch` in c, although with the addition of pattern matching and the lack of fallthrough

```rust
let level = Level::Shot{show: "dev01".into(), seq: "rd".into(), "0001".into()};

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
```

A couple of things to note:
- by default, matches must be exhaustive. You cannot leave off a pattern. The compiler will bark at you.
- You can use '_' to match all remaining patterns in you want to.
- Ownership is dictated by the ownership of the variable that you are matching against. If it is owned, you can extract owned components. Otherwise, you will be dealing with references
- you can explicitly note a reference using the `ref` keyword
- you can explicitly make the ref mutable by using the `mut ref` keywords, provided the variable is mutable to begin with.
- match is an expression. Each arm returns the value of the last expression in the branch. You can add parens to group multiple expressions together if you want. If the last statement in the branch ends in a semi-colon, then the return vaue is the unit `()`. 
- you can nest match expressions
- you can assign the return value to a variable (and often will).

```rust
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
```

## Match Guards

You can refine the match arms using guards. For example

```rust
let id = 3;
match id {
    1 | 2 | 3 => println!("small"),
    4 | 5 | 6 => println!("med"),
    _ => println!("large") ,
}
```

## enums may have methods, just like structs. 
Destructuring is very useful, but that is only half the power of these enums. The other killer feature is that enums may define methods and associated functions just like structs.

```rust
impl Level {
    /// print the content
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
            Self::Show(show) =>show.clone(),
            Self::Seq { show,.. } => show.clone(),
            Self::Shot { show, .. } => show.clone(),
        }
    }
}
```

### Common enums
The standard library defines a couple of important enum