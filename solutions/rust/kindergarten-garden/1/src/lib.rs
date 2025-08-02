const STUDENTS: [&str; 12] = [
    "Alice",
    "Bob",
    "Charlie",
    "David",
    "Eve",
    "Fred",
    "Ginny",
    "Harriet",
    "Ileana",
    "Joseph",
    "Kincaid",
    "Larry"
];

static GRASS: &str = "grass";
static CLOVER: &str = "clover";
static RADISH: &str = "radishes";
static VIOLET: &str = "violets";

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let rank = STUDENTS.iter().position(|s| s == &student).expect("the student rank");
    let pos = rank * 2;
    diagram.split("\n").flat_map(|row|
        row.chars()
            .skip(pos)
            .take(2)
            .filter_map(|c| Some(match c {
                'G' => GRASS,
                'C' => CLOVER,
                'R' => RADISH,
                'V' => VIOLET,
                _=> return None,
            }))
    )
    .collect()
}
