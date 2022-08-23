struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 85 },
        Test { score: 90 },
        Test { score: 100}
    ];

    for test in my_scores {
        println!("Score = {:?}", test.score);
    }
}
