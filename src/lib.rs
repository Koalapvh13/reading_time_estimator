pub fn reading_time_in_minutes(text: &str) -> i32{
    let words_counted: i32 = words_count(text);
    let calc: f32 = (words_counted as f32)/(160 as f32);
    calc as i32
}

pub fn reading_time_in_seconds(text: &str) -> i32{
    let words_counted: i32 = words_count(text);
    let calc: f32 = (words_counted as f32)/(160 as f32);
    (calc*(60 as f32)) as i32
}

fn words_count(text: &str) -> i32{
    let words: Vec<&str> = text.split(' ').collect();
    let count: Vec<&str> = words.into_iter().filter(|word| word != &" ").collect();
    count.len() as i32
}
