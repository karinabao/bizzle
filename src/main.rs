use rocket::{get, routes, launch, Build, Rocket, form::{Form, FromForm}, State};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rand::seq::SliceRandom;
use std::{sync::RwLock, fs::File};
use num_format::{Locale, ToFormattedString};

#[macro_use] extern crate rocket;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Company {
    name: String,
    ticker: String,
    rank: u32,
    year: u32,
    industry: String,
    sector: String,
    headquarters_state: String,
    headquarters_city: String,
    marketcap: u64,
    revenue_mil: u64,
    profit_mil: u64,
    asset_mil: u64,
    employees: u64,
    description: String,
}

#[derive(Debug)]
struct AppState {
    selected_company: RwLock<Option<Company>>,
    total_games: RwLock<u32>,
    correct_guesses: RwLock<u32>,
    incorrect_guesses: RwLock<u32>,
    total_time: RwLock<u32>, 
}

#[derive(FromForm)]
struct Guess {
    guess_type: String,
}

#[derive(Deserialize)]
struct TimePayload {
    time: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stats {
    total_games: u32,
    correct_guesses: u32,
    incorrect_guesses: u32,
    total_time: u32,
}

fn read_csv() -> Result<Vec<Company>, std::io::Error> {
    let file = File::open("data/f500data-shortlist.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut companies = Vec::new();

    for result in rdr.deserialize() {
        let record: Company = result?;
        companies.push(record);
    }
    Ok(companies)
}

fn get_random_company(companies: &Vec<Company>) -> Option<&Company> {
    let mut rng = rand::thread_rng();
    companies.choose(&mut rng)
}

#[get("/company")]
fn company(state: &State<AppState>) -> Result<Json<Company>, &'static str> {
    let companies = read_csv().map_err(|_| "Failed to read CSV")?;
    let company = get_random_company(&companies).ok_or("No companies available")?;
    
    let mut selected_company = state.selected_company.write().unwrap();
    *selected_company = Some(company.clone());

    Ok(Json(company.clone()))
}

fn format_billion(value: u64) -> String {
    format!("${:.1}B", value as f64 / 1_000.0)
}

#[post("/submit_guess", data = "<guess>")]
fn submit_guess(guess: Form<Guess>, state: &State<AppState>) -> String {
    let guess = guess.into_inner();

    let selected_company = state.selected_company.read().unwrap();

    if let Some(ref company) = *selected_company {
        let market_cap_estimate = if company.rank <= 250 { 40_000 } else { 10_000 }; // value in millions
        let revenue_estimate = if company.rank <= 250 { 30_000 } else { 7_500 }; // value in millions
        let profit_estimate = if company.rank <= 250 { 10_000 } else { 2_500 }; // value in millions
        let assets_estimate = if company.rank <= 250 { 25_000 } else { 6_000 }; // value in millions
        let employees_estimate = if company.rank <= 250 { 30_000 } else { 7_500 }; // raw value

        let formatted_market_cap_billion = format_billion(company.marketcap);
        let formatted_revenue_billion = format_billion(company.revenue_mil);
        let formatted_profit_billion = format_billion(company.profit_mil);
        let formatted_assets_billion = format!("${:.1}B", company.asset_mil as f64 / 1_000.0);
        let formatted_employees = format!("{}", company.employees.to_formatted_string(&Locale::en));

        let mut correct_guesses = state.correct_guesses.write().unwrap();
        let mut incorrect_guesses = state.incorrect_guesses.write().unwrap();
        let mut total_games = state.total_games.write().unwrap();

        let result = match guess.guess_type.as_str() {
            "market_cap_higher" => {
                if company.marketcap > market_cap_estimate {
                    *correct_guesses += 1;
                    format!("Correct! The actual market cap of {} is {} which is higher than ${:.1}B", company.name, formatted_market_cap_billion, market_cap_estimate as f64 / 1_000.0)
                } else {
                    *incorrect_guesses += 1;
                    format!("Incorrect! The actual market cap of {} is {} which is lower than ${:.1}B", company.name, formatted_market_cap_billion, market_cap_estimate as f64 / 1_000.0)
                }
            }
            "market_cap_lower" => {
                if company.marketcap < market_cap_estimate {
                    *correct_guesses += 1;
                    format!("Correct! The actual market cap of {} is {} which is lower than ${:.1}B", company.name, formatted_market_cap_billion, market_cap_estimate as f64 / 1_000.0)
                } else {
                    *incorrect_guesses += 1;
                    format!("Incorrect! The actual market cap of {} is {} which is higher than ${:.1}B", company.name, formatted_market_cap_billion, market_cap_estimate as f64 / 1_000.0)
                }
            }
            "revenue_higher" => {
                if company.revenue_mil > revenue_estimate {
                    *correct_guesses += 1;
                    format!("Correct! The actual revenue of {} is {} which is higher than ${:.1}B", company.name, formatted_revenue_billion, revenue_estimate as f64 / 1_000.0)
                } else {
                    *incorrect_guesses += 1;

                    format!("Incorrect! The actual revenue of {} is {} which is lower than ${:.1}B", company.name, formatted_revenue_billion, revenue_estimate as f64 / 1_000.0)
                }
            }
            "revenue_lower" => {
                if company.revenue_mil < revenue_estimate {
                    *correct_guesses += 1;
                    format!("Correct! The actual revenue of {} is {} which is lower than ${:.1}B", company.name, formatted_revenue_billion, revenue_estimate as f64 / 1_000.0)
                } else {
                    *incorrect_guesses += 1;
                    format!("Incorrect! The actual revenue of {} is {} which is higher than ${:.1}B", company.name, formatted_revenue_billion, revenue_estimate as f64 / 1_000.0)
                }
            }

            "profit_higher" => {
                if company.profit_mil > profit_estimate {
                    *correct_guesses += 1;
                    println!("correct guess + 1");
                    format!("Correct! The actual profit of {} is {} which is higher than ${:.1}B", company.name, formatted_profit_billion, profit_estimate as f64 / 1_000.0)
                } else {
                    *incorrect_guesses += 1;

                    format!("Incorrect! The actual profit of {} is {} which is lower than ${:.1}B", company.name, formatted_profit_billion, profit_estimate as f64 / 1_000.0)
                }
            }
            "profit_lower" => {
                if company.profit_mil < profit_estimate {
                    *correct_guesses += 1;
                    println!("correct guess + 1");
                    format!("Correct! The actual profit of {} is {} which is lower than ${:.1}B", company.name, formatted_profit_billion, profit_estimate as f64 / 1_000.0)
                } else {
                    *incorrect_guesses += 1;

                    format!("Incorrect! The actual profit of {} is {} which is higher than ${:.1}B", company.name, formatted_profit_billion, profit_estimate as f64 / 1_000.0)
                }
            }
            "assets_higher" => {
                if company.asset_mil > assets_estimate {
                    *correct_guesses += 1;
                    println!("correct guess + 1");
                    format!("Correct! The actual assets of {} are {} which is higher than ${:.1}B", company.name, formatted_assets_billion, assets_estimate as f64 / 1_000.0)
                } else {
                    *incorrect_guesses += 1;

                    format!("Incorrect! The actual assets of {} are {} which is lower than ${:.1}B", company.name, formatted_assets_billion, assets_estimate as f64 / 1_000.0)
                }
            }
            "assets_lower" => {
                if company.asset_mil < assets_estimate {
                    *correct_guesses += 1;
                    println!("correct guess + 1");
                    format!("Correct! The actual assets of {} are {} which is lower than ${:.1}B", company.name, formatted_assets_billion, assets_estimate as f64 / 1_000.0)
                } else {
                    *incorrect_guesses += 1;
                    format!("Incorrect! The actual assets of {} are {} which is higher than ${:.1}B", company.name, formatted_assets_billion, assets_estimate as f64 / 1_000.0)
                }
            }
            "employees_higher" => {
                if company.employees > employees_estimate {
                    *correct_guesses += 1;
                    println!("correct guess + 1");
                    format!("Correct! The actual employees of {} is {} which is higher than {}", company.name, formatted_employees, employees_estimate.to_formatted_string(&Locale::en))
                } else {
                    *incorrect_guesses += 1;
                    format!("Incorrect! The actual employees of {} is {} which is lower than {}", company.name, formatted_employees, employees_estimate.to_formatted_string(&Locale::en))
                }
            }
            "employees_lower" => {
                if company.employees < employees_estimate {
                    *correct_guesses += 1;
                    println!("correct guess + 1");
                    format!("Correct! The actual employees of {} is {} which is lower than {}", company.name, formatted_employees, employees_estimate.to_formatted_string(&Locale::en))
                } else {
                    *incorrect_guesses += 1;
                    format!("Incorrect! The actual employees of {} is {} which is higher than {}", company.name, formatted_employees, employees_estimate.to_formatted_string(&Locale::en))
                }
            }
            _ => "Invalid guess".to_string(),
        };
        if (*correct_guesses + *incorrect_guesses) % 5 == 0 {
            *total_games += 1;
        }

        return result;
    }
    "Failed to process the guess".to_string()
}

#[get("/stats")]
fn get_stats(state: &State<AppState>) -> Result<Json<Stats>, &'static str> {
    let total_games = *state.total_games.read().unwrap();
    let correct_guesses = *state.correct_guesses.read().unwrap();
    let incorrect_guesses = *state.incorrect_guesses.read().unwrap();
    let total_time = *state.total_time.read().unwrap();

    let stats = Stats {
        total_games,
        correct_guesses,
        incorrect_guesses,
        total_time
    };

    Ok(Json(stats))
}

#[post("/stats", data = "<payload>")]
fn update_stats(state: &State<AppState>, payload: Json<TimePayload>) -> Result<Json<Stats>, &'static str> {
    {
        let mut total_time = state.total_time.write().unwrap();
        *total_time += payload.time;
    }

    let total_time = *state.total_time.read().unwrap();
    let total_games = *state.total_games.read().unwrap();
    let incorrect_guesses = *state.incorrect_guesses.read().unwrap();
    let correct_guesses = *state.correct_guesses.read().unwrap();

    let stats = Stats {
        total_games,
        correct_guesses,
        incorrect_guesses,
        total_time
    };

    Ok(Json(stats))
}




#[launch]
fn rocket() -> Rocket<Build> {
    let state = AppState {
        selected_company: RwLock::new(None),
        total_games: RwLock::new(0),
        correct_guesses: RwLock::new(0),
        incorrect_guesses: RwLock::new(0),
        total_time: RwLock::new(0),
    };
    rocket::build()
        .manage(state)
        .mount("/", routes![company, submit_guess, get_stats, update_stats])
        .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static")))
}
