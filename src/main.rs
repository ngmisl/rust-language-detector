//uncomment for specific languages, much faster

// use lingua::Language::{English, French, German, Spanish};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use std::io;

fn main() {
    println!("Enter Text you want the language detected for: ");
    let mut lan = String::new();
    io::stdin()
        .read_line(&mut lan)
        .expect("Failed to read line");

    //    let languages = vec![English, French, German, Spanish];
    //    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();

    let detector: LanguageDetector = LanguageDetectorBuilder::from_all_languages().build(); // <- comment this when moving to specific languages
    let detected_language: Option<Language> = detector.detect_language_of(&lan);

    println!("Detected language: {:?}", detected_language);
}
