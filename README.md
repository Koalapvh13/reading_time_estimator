# reading_time_estimator
Package to estimate the reading time from a text. Medium's like.

## Usage

```
extern crate reading_time_estimator;
fn main() {
    println!(
        "{} koala",
        reading_time_estimator::reading_time_in_minutes("[text-to-count]")
    );
    println!(
        "{} koala",
        reading_time_estimator::reading_time_in_seconds("[text-to-count]")
    );
}

```

## Install

```
reading_time_estimator = "0.1.0"
```
