use std::env::temp_dir;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, LineWriter, Write};
use std::thread;
use std::time::Duration;

use uuid::Uuid;
use fancy_hangman::lang::locale::{AppLanguage, replace_unicode, get_app_language, parse_app_language};
use fancy_hangman::dictionary::{Dictionary, DictionaryEntry, get_dictionary};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

/// A maintenance tool for wordle
#[derive(Parser)]
struct Arguments {
    source_file: String,
    language: Option<String>,
    dictionary: Option<String>
}
fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    let app_language = match args.language {
        None => get_app_language(),
        Some(flag) => parse_app_language(flag.as_str())
    };

    let dictionary: Box<dyn Dictionary> = match args.dictionary {
        None => get_dictionary(app_language, String::from("text")),
        Some(flag) => get_dictionary(app_language, flag)
    };

    let progress_bar = setup_spinner();
    progress_bar.set_message(format!("polishing file {}", &args.source_file));

    let source_file = polish(&args.source_file, app_language)?;

    thread::sleep(Duration::from_secs(4));

    progress_bar.finish_and_clear();
    progress_bar.set_message("Importing...");

    let counter = import(source_file, dictionary)?;

    thread::sleep(Duration::from_secs(4));

    progress_bar.set_message(format!("Finished. Imported {} words", counter));

    println!("Added {} words to the dictionary!", counter);

    Ok(())
}

/// Read raw word list from source_path and polish with matching app_language strategy.
/// The polished list is then written to a temporary file located in the tmp directory of the filesystem.
///
/// See [temp_dir] documentation for more information.
///
/// # Arguments
///
/// * `src_path` - A string slice that holds the path of the file you want to import on the filesystem
/// * `app_language` - The language of the imported words. See [AppLanguage]
fn polish(source_path: &str, app_language: AppLanguage) -> Result<String, Error> {
    let tmp_file_name = format!("{}/{}.txt", temp_dir().to_str().unwrap(), Uuid::new_v4());
    let out_file: Result<File, Error> = File::create(&tmp_file_name);

    match out_file {
        Ok(out_file) => {
            let buf_reader = BufReader::new(File::open(source_path).unwrap());
            let mut writer: LineWriter<File> = LineWriter::new(out_file);

            println!("processing file {}", source_path);

            for line_result in buf_reader.lines() {
                let polished = replace_unicode(line_result.unwrap().to_lowercase().as_str(), app_language);

                if polished.len() == 5 {
                    print!(".");

                    writer.write(polished.as_ref())?;
                    writer.write(b"\n")?;
                }
            }

            println!("finished polishing");

            Ok(tmp_file_name)
        }
        Err(error) => Err(error)
    }
}

/// Import temporary file created by [polish] into the dictionary.
/// Avoid duplicates when inserting a [WordEntry] into the dictionary.
///
/// # Arguments
///
/// * `tmp_file_name` - A String that holds the name of the temp file created
fn import(tmp_file_name: String, dictionary: Box<dyn Dictionary>) -> Result<i32, Error> {
    let buf_reader = BufReader::new(File::open(tmp_file_name).unwrap());

    let mut counter = 0;
    for line_result in buf_reader.lines() {
        let line = line_result.unwrap();

        match dictionary.create_word(DictionaryEntry { word: line.to_lowercase() }) {
            None => {},
            Some(_) => {
                counter += 1;
            }
        }

    }

    Ok(counter)
}

fn setup_spinner() -> ProgressBar {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.enable_steady_tick(120);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .progress_chars("#>-"));

    progress_bar
}