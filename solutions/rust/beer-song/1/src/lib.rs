pub fn sing(start: u32, end: u32) -> String {
    (end..=start).into_iter().map(verse).rev().collect::<Vec<_>>().join("\n")
}

pub fn verse(n: u32) -> String {
    if n > 0 {
        format!(
            "{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n",
            bottles(n),
            bottles(n),
            if n > 1 { "one" } else { "it" },
            bottles(n-1)
        )
    } else {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    }
}

fn bottles(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        n => format!("{n} bottles"),
    }
}
