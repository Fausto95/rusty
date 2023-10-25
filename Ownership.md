# Ownership

### Moving 

It allows Rust to execute code in a performant way, ensures that compiled code executes under various circumstances to avoid memory leaks.


Memory leak is when a program fails to track which memory is being used.

In Rust's owner, a memory can be `moved` or `borrowed` and the responsible for the memory should clean it.

### Borrowing 

```rust

enum WeekDay {
    Monday,
    Friday,
}

fn display_week_day(day: WeekDay) {
    match day {
        WeekDay::Monday => println!("Today is Monday"),
        WeekDay::Friday => println!("Today is Friday"),
    }
}

fn main() {
    let day = WeekDay::Monday;

    display_week_day(day);
    display_week_day(day);
}

```

If we run the above code, it will error because when we create the `day` variable, it's owned by the `main` function. When we invoke `display_week_day(day)` we basic move the memory - meaning the `day` var is now owned by the `display_week_day` function, so after its first execution, the variable gets distroyed and the memory is freed, so is no longer available to be used.

```rust

enum WeekDay {
    Monday,
    Friday,
}

fn display_week_day(day: &WeekDay) {
    match day {
        WeekDay::Monday => println!("Today is Monday"),
        WeekDay::Friday => println!("Today is Friday"),
    }
}

fn main() {
    let day = WeekDay::Monday;

    display_week_day(&day);
    display_week_day(&day);
}

```

Now in the above example, we are borrowing the memory, so the `day` variable is still being owned by the `main` function, only it can free it after using it.