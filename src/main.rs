use std::{fs::File, io::Error};
use rocket::{serde::{Deserialize, Serialize, json::Json}, get, post, launch, routes, State, form::Form};
use std::sync::RwLock;
use rocket::fs::NamedFile;
use std::path::Path;
use rand::seq::SliceRandom;
use rocket::fs::FileServer;
use rand::thread_rng;
use num_format::{Locale, ToFormattedString};

#[macro_use] extern crate rocket;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Company {
    name: String,
    rank: u32,
    market_value_mil: u64,
    industry: String,
    description: String,
}

#[derive(FromForm)]
struct Guess {
    guess: f64,
    guessNumber: u8,
}

struct GameState {
    companies: RwLock<Vec<Company>>,
    current_company: RwLock<Option<Company>>,
    first_guess: RwLock<Option<u64>>,
    second_guess: RwLock<Option<u64>>,
    third_guess: RwLock<Option<u64>>,
    fourth_guess: RwLock<Option<u64>>,
}

fn read_csv() -> Result<Vec<Company>, Error> {
    let file = File::open("data/f500data.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    let mut companies = Vec::new();

    for result in reader.deserialize() {
        let record: Company = result?;
        companies.push(record);
    }
    Ok(companies)
}

fn get_random_company(companies: &Vec<Company>) -> Option<&Company> {
    let mut rng = thread_rng();
    companies.choose(&mut rng)
}

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open(Path::new("static/index.html")).await.unwrap()
}

#[get("/company")]
fn random_company(state: &State<GameState>) -> Result<Json<Company>, &'static str> {
    let companies = state.companies.read().unwrap();
    let company = get_random_company(&companies).ok_or("No companies available")?;
    let mut current_company = state.current_company.write().unwrap();
    *current_company = Some(company.clone());
    let mut first_guess = state.first_guess.write().unwrap();
    *first_guess = None;
    let mut second_guess = state.second_guess.write().unwrap();
    *second_guess = None;
    let mut third_guess = state.third_guess.write().unwrap();
    *third_guess = None;
    let mut fourth_guess = state.fourth_guess.write().unwrap();
    *fourth_guess = None;
    Ok(Json(company.clone()))
}

fn format_billion(value: u64) -> String {
    let billions = value as f64 / 1_000_000_000.0;
    if billions < 1.0 {
        format!("${:.2} B", billions)
    } else {
        let formatted_billions = billions.round() as u64;
        format!("${} B", formatted_billions.to_formatted_string(&Locale::en))
    }
}

fn check_guess(actual_value: u64, guess: u64, previous_guess: Option<u64>) -> String {
    if let Some(prev_guess) = previous_guess {
        let previous_diff = (prev_guess as i64 - actual_value as i64).abs() as u64;
        let current_diff = (guess as i64 - actual_value as i64).abs() as u64;

        if current_diff < previous_diff {
            "You're getting warmer! Try again:".to_string()
        } else {
            "You're getting cooler! Try again:".to_string()
        }
    } else {
        if (guess as f64 / actual_value as f64) < 2.0 && (actual_value as f64 / guess as f64) > 0.5 {
            "You're warm! Try again:".to_string()
        } else {
            "You're cold! Try again:".to_string()
        }
    }
}

#[post("/submit_guess", data = "<guess_form>")]
fn submit_guess(state: &State<GameState>, guess_form: Form<Guess>) -> String {
    let guess_in_billion = guess_form.guess;
    let guess = (guess_in_billion * 1_000_000_000.0) as u64;
    let guess_number = guess_form.guessNumber;
    let current_company = state.current_company.read().unwrap();
    let mut first_guess = state.first_guess.write().unwrap();
    let mut second_guess = state.second_guess.write().unwrap();
    let mut third_guess = state.third_guess.write().unwrap();
    let mut fourth_guess = state.fourth_guess.write().unwrap();
    
    if let Some(ref company) = *current_company {
        let actual_value = company.market_value_mil * 1_000_000;  // Convert to dollars

        info!("Company: {:?}", company);
        info!("Actual market value (in millions): {}", company.market_value_mil);
        info!("Actual market value (in billions): {:.2}", actual_value as f64 / 1_000_000_000.0);
        info!("Guessed market value (in billions): {:.2}", guess_in_billion);

        let rounded_actual = (actual_value as f64 / 1_000_000_000.0).round() as u64;
        let rounded_guess = guess_in_billion.round() as u64;

        if rounded_guess == rounded_actual {
            return format!(
                "Congratulations! Your guess of ${:.2} B is correct!\nDescription: {}\nFortune 500 Rank: {}\nIndustry: {}",
                guess_in_billion,
                company.description,
                company.rank,
                company.industry
            );
        }

        let hint = match guess_number {
            1 => format!("\nHint 1: Industry: {}", company.industry),
            2 => format!("\nHint 2: Description: {}", company.description),
            3 => format!("\nFinal hint: Fortune 500 rank: {}", company.rank),
            _ => String::new(),
        };

        match guess_number {
            1 => {
                *first_guess = Some(guess);
                format!("{}{}", check_guess(actual_value, guess, None), hint)
            },
            2 => {
                let feedback = check_guess(actual_value, guess, *first_guess);
                *second_guess = Some(guess);
                format!("{}{}", feedback, hint)
            },
            3 => {
                let feedback = check_guess(actual_value, guess, *second_guess);
                *third_guess = Some(guess);
                format!("{}{}", feedback, hint)
            },
            4 => {
                *fourth_guess = Some(guess);
                let difference = (actual_value as i64 - guess as i64).abs() as u64;
                format!(
                    "You guessed: {}. The actual market value is: {}. The difference is: {}.",
                    format_billion(guess),
                    format_billion(actual_value),
                    format_billion(difference)
                )
            },
            _ => "Invalid guess number.".to_string(),
        }
    } else {
        "No company selected.".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    let companies = read_csv().expect("Failed to read CSV");
    let game_state = GameState {
        companies: RwLock::new(companies),
        current_company: RwLock::new(None),
        first_guess: RwLock::new(None),
        second_guess: RwLock::new(None),
        third_guess: RwLock::new(None),
        fourth_guess: RwLock::new(None),
    };

    rocket::build()
        .mount("/", routes![index, random_company, submit_guess])
        .manage(game_state)
        .mount("/static", FileServer::from("static"))
}
