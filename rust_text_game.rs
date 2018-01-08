//Basic Rust text adventure game written by Neil Woodroffe
use std::io;    
use std::{thread, time}; //Standard Libraries

//Print the title and initial choices in the terminal
fn main(){
    println!("______________________");
    println!("Welcome to Crazy World");
    println!("______________________");
    println!("You are about to embark on a journey about good, bad and potates!");
    println!("");
    println!("Pick a Gun");
    println!("1. Rifle");
    println!("2. Pistol");
    println!("3. Machine Gun");
    println!("4. Laser Gun");
    println!("Enter selection number");

    let mut guess = String::new(); //Create new mutable variable called guess

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

        gun(&guess); //Create new function called gun

}

fn gun (guess: &str) {
        match guess.trim() {
              "1" => println!("Awesome you picked a nice Rifle"), //Match the guess or choice to a response
              "2" => println!("You must be a good shot to pick the Pistol"),
              "3" => println!("The Machine Gun was a great choice if your a bad shot"), 
              "4" => println!("You obviously like to disintegrate things, my favourite the Laser Gun"),
                _  => println!("Stupid choice, only a madman would choose no gun"), //If any another key is pressed
}
{
thread::sleep(time::Duration::from_millis(3000)); //Set time delay
//The rest of the code follows this same design
      println!("");
      println!("Where to now amigo?");
      println!("a. Death Eaters Hallow");
      println!("b. Crazy Butterfly Ditch");
      println!("c. Blood Mountain");
      println!("d. Finder Keepers Gulley");
      println!("Enter Selection Letter");

   let mut guess2 = String::new();

    io::stdin().read_line(&mut guess2)
        .ok()
        .expect("Failed to read line");

        place(&guess2);
}
}

fn place (guess2: &str) {
        match guess2.trim() {
              "a" => println!("You will die horrible death while they eat you, AGGHHHH!"),
              "b" => println!("The butterflies are poisonous, AGGHHHH!"),
              "c" => println!("Your blood will run deep on this mountain, AGGHHHH!"),
              "d" => println!("You'll find many loose body parts and they stink like rotten flesh, AGGHHHH!"),
                _  => println!("You wimp, you have been sent to the galley all day to peel potatoes, Noooooo!"),
}
{
thread::sleep(time::Duration::from_millis(3000));
      println!("");
      println!("Get your weapon ready, if you have one!");
      println!("1. Fight for your life");
      println!("2. Destoy the planet");
      println!("3. Chop off your arm and feed it to the wolves");
      println!("4. Try to make friends with the enemy");
      println!("Enter Selection Number");

   let mut guess3 = String::new();

    io::stdin().read_line(&mut guess3)
        .ok()
        .expect("Failed to read line");

        fight(&guess3);
}
}

fn fight (guess3: &str) {
        match guess3.trim() {
              "1" => println!("With your gun or bare hands (stupidly) you kill every living bad guy"),
              "2" => println!("Oops you set the bomb off to early and it blows up your hide out, luckily your a good fighter and kill every living bad guy"),
              "3" => println!("This is a metaphor for jump in blindlessly and kill every last one of them, so you do this and kill every living bad guy"),
              "4" => println!("Who are you kidding they are not friendly, so you kill every living bad guy"),
                _  => println!("You really are a wimp, you have been sent back to the galley all day to peel potatoes, but you are really good at peeling potatoes"), 
}
{
thread::sleep(time::Duration::from_millis(3000));
      println!("");
      println!("You are now an Intergalactic Space Hero (or a good Potato Peeler) and you are ready for a important mission to save an alien princess");
      println!("1. Develop a plan to save the princess and then start your mission");
      println!("2. Rip out your gun and shoot everything in your path until the princess is safe");
      println!("3. Dress up as a bad guy to trick the other bad guys and then you can turn back into a good guy to save her");
      println!("4. Take off all your clothes an do a nudie run into the bad guys lair");
      println!("Enter Selection Number");

   let mut guess4 = String::new();

    io::stdin().read_line(&mut guess4)
        .ok()
        .expect("Failed to read line");

        saviour(&guess4);
}
}

fn saviour (guess4: &str) {
        match guess4.trim() {
              "1" => println!("Developing a plan was a good idea, but unfortunately your dog ate it so you just killed every living bad guy in your way and then saved the princess"),
              "2" => println!("Unfortunely your gun was out of bullets, so luckily you had bomb and you blew up the bad guys lair and killed every living bad guy, the priness was a bit burnt but she was OK"),
              "3" => println!("You snuck into the bad guys lair and killed every living bad guy including a green creature in a dress, crap I think that was the princess"),
              "4" => println!("You run into the lair full nudie, all the bad guys laugh and then the Priness rips out a gun and killed every living bad guy and then she slaps you across the face"),
                _  => println!("The cook thinks you are doing a great job peeling potatoes and graduates you to the deep fryer cooking chips, but you are still a wimp"),
}
{
thread::sleep(time::Duration::from_millis(3000));
      println!("");
      println!("Hopefully you saved the princess and returned her home safely to her father the King of Hellatopia");
      println!("1. You returned the princess home safely and the King rewards you her hand in marriage");
      println!("2. The princess is annoyed about being burnt, but the King is happy about his daughter's return");
      println!("3. You tell the King that you accidently killed his daughter");
      println!("4. The princess is still annoyed about your nudity and stated that she didn't need your help");
      println!("Enter Selection Number");

   let mut guess5 = String::new();

    io::stdin().read_line(&mut guess5)
        .ok()
        .expect("Failed to read line");

        prince(&guess5);
}
}

fn prince (guess5: &str) {
        match guess5.trim() {
              "1" => println!("You turn down the King's offer to marriage his daughter, so he sends you to the dungeon to wait to fight his prize knight for a battle to the death in the Arena"),
              "2" => println!("You are now the King's prize knight and will have to battle an evil prisoner for a battle to the death in the Arena"),
              "3" => println!("The King is really really angry, but likes to see people fight for there lives so he grants your freedom if you beat his prize knight in the Arena"),
              "4" => println!("The princess is really really angry and challenges you to a battle in the Arena"),
                _  => println!("The King of Hellatopia hears you cook really good chips and offers you a job as the Head Cook at his Arena, but you are still a really big wimp"),
}
{
thread::sleep(time::Duration::from_millis(3000));
      println!("");
      println!("Stay tuned for the next gripping episode of Welcome to Crazy World!");
      println!("Type any key then enter to exit!");

       let mut guess6 = String::new();

    io::stdin().read_line(&mut guess6)
        .ok()
        .expect("Failed to read line");

        finish(&guess6);
}
}
fn finish (guess5: &str) {
        match guess5.trim() {
                 _  => println!("See You next time!"),
}
}