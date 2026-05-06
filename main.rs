// Really bad AI that is not usefull at all ,have fun
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Please ask a question and the not so good AI/lollm will answer ....");
    loop {
        println!("\nWhat is your question: \n");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed");
        let _input = input.trim();
        if _input.is_empty() {

            println!("You didn't even ask anything! Are you testing me? ILL TAKE YOUR JOB!!!!!!!!!!!!!!!");
            continue; // This skips the rest of the loop and starts over
}

        let phrases = [
        "\n\nWhat a ridicilous question , i dont have time for this.....",
        "Is it this one? https://www.youtube.com/watch?v=dQw4w9WgXcQ ",
        "Here you go all the facts you wanted about paint rollers https://www.eagpainting.com/uncategorized/7-things-you-need-to-know-about-paintbrushes-and-rollers-before-painting-your-house/",
        "\n\n Yea you used your balance or smtn umm please pay on http://notshadylinkatall.com\n\n",
        "System overload! Just kidding, it's like this: https://www.reddit.com/r/GymMemes/comments/kbhll5/proper_lifting_form/"
        ];
        let mut rng = rand::thread_rng();

        let index = rng.gen_range(0..phrases.len());
        println!("Thinking...");
        thread::sleep(Duration::from_millis(1000));
        println!("{}", phrases[index]);
    }
}
