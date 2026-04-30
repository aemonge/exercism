fn index_of(student: &str) -> usize {
    match student {
        "Alice" => 0,
        "Bob" => 2,
        "Charlie" => 4,
        "David" => 6,
        "Eve" => 8,
        "Fred" => 10,
        "Ginny" => 12,
        "Harriet" => 14,
        "Ileana" => 16,
        "Joseph" => 18,
        "Kincaid" => 20,
        "Larry" => 22,
        _ => 0,
    }
}

fn code_to_plant(code: char) -> &'static str {
    match code {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => ".",
    }
}

fn codes_to_plants(codes: &str) -> Vec<&'static str> {
    let mut it = codes.chars();

    let first = code_to_plant(
        it.next()
            .unwrap(),
    );
    let second = code_to_plant(
        it.next()
            .unwrap(),
    );

    vec![first, second]
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let l = diagram.len();
    let (fist_row, second_row) = diagram.split_at(l / 2);
    let second_row = second_row.trim_start();

    let i = index_of(student);
    let Some(first) = fist_row.get(i..=i + 1) else {
        return vec![".."];
    };
    let Some(second) = second_row.get(i..=i + 1) else {
        return vec![".."];
    };

    [codes_to_plants(first), codes_to_plants(second)].concat()
}
