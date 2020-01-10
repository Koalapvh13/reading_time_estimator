pub fn reading_time_in_minutes(text: &str) -> i32{
    words_count(text);
    55
}

fn words_count(text: &str) -> usize{
    let words: Vec<&str> = text.split(' ').collect();
    let count: Vec<&str> = words.into_iter().filter(|word| word != &" ").collect();
    println!("{}", count.len());
    count.len()
}