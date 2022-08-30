fn main() {
    let edad = 20;

    let mensaje_edad = match edad {
        18 => "You have 18 years",
        17 => "You still access, coming soon",
        0..16 => "you are still a minor",
        19 | 20 | 21 => "drink alcohol in moderation",
        _ => "Your age i don't now"
    };

    println!("The message is {}", mensaje_edad);
}