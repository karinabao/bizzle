use rocket::{get, post, routes, launch, Build, Rocket, form::{Form, FromForm}, State};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rand::seq::SliceRandom;
use std::{sync::RwLock, collections::HashMap, fs::File};
use num_format::{Locale, ToFormattedString};
use csv::Reader;
use uuid::Uuid;
use rocket::http::{Cookie, CookieJar};
use std::sync::Arc;

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

#[derive(Debug, Default, Clone)]
struct UserStats {
    total_games: u32,
    correct_guesses: u32,
    incorrect_guesses: u32,
    total_time: u32,
}

#[derive(Debug, Default)]
struct AppState {
    selected_company: RwLock<Option<Company>>,
    user_stats: RwLock<HashMap<String, UserStats>>,
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
    let mut rdr = Reader::from_reader(file);
    let mut companies = Vec::new();

    for result in rdr.deserialize() {
        let record: Company = result?;
        companies.push(record);
    }
    Ok(companies)
}

fn get_random_company(companies: &[Company]) -> Option<&Company> {
    let mut rng = rand::thread_rng();
    companies.choose(&mut rng)
}

#[get("/company")]
fn company(state: &State<AppState>, cookies: &CookieJar<'_>) -> Result<Json<Company>, &'static str> {
    let companies = read_csv().map_err(|_| "Failed to read CSV")?;
    let company = get_random_company(&companies).ok_or("No companies available")?;

    let mut selected_company = state.selected_company.write().unwrap();
    *selected_company = Some(company.clone());

    let user_id = get_user_id(cookies);
    cookies.add(Cookie::new("user_id", user_id.clone()));

    Ok(Json(company.clone()))
}

fn format_billion(value: u64) -> String {
    format!("${:.1}B", value as f64 / 1_000.0)
}

fn update_guess_counts(state: &State<AppState>, user_id: &str, correct: bool) {
    let mut user_stats = state.user_stats.write().unwrap();
    let stats = user_stats.entry(user_id.to_string()).or_default();

    if correct {
        stats.correct_guesses += 1;
    } else {
        stats.incorrect_guesses += 1;
    }

    if (stats.correct_guesses + stats.incorrect_guesses) % 5 == 0 {
        stats.total_games += 1;
    }
}

fn evaluate_guess(company: &Company, guess_type: &str, estimate: u64, actual: u64, unit: &str) -> String {
    let is_correct = match guess_type {
        "higher" => actual > estimate,
        "lower" => actual < estimate,
        _ => return "Invalid guess type".to_string(),
    };

    let formatted_value = match unit {
        "B" => format_billion(actual),
        _ => actual.to_formatted_string(&Locale::en),
    };

    format!(
        "{}! Actual: {}",
        if is_correct { "Correct" } else { "Incorrect" },
        formatted_value
    )
}

#[post("/submit_guess", data = "<guess>")]
fn submit_guess(guess: Form<Guess>, state: &State<AppState>, cookies: &CookieJar<'_>) -> String {
    let guess = guess.into_inner();
    let selected_company = state.selected_company.read().unwrap();

    let user_id = get_user_id(cookies);
    cookies.add(Cookie::new("user_id", user_id.clone()));

    if let Some(ref company) = *selected_company {
        let estimates = if company.rank <= 250 {
            (40_000, 30_000, 10_000, 25_000, 30_000) // values in millions for market cap, revenue, profit, assets and raw value for employees
        } else {
            (10_000, 7_500, 2_500, 6_000, 7_500)
        };

        let (marketcap_estimate, revenue_estimate, profit_estimate, assets_estimate, employees_estimate) = estimates;

        let result = match guess.guess_type.as_str() {
            "marketcap_higher" | "marketcap_lower" => {
                evaluate_guess(company, &guess.guess_type.split('_').last().unwrap(), marketcap_estimate, company.marketcap, "B")
            }
            "revenue_higher" | "revenue_lower" => {
                evaluate_guess(company, &guess.guess_type.split('_').last().unwrap(), revenue_estimate, company.revenue_mil, "B")
            }
            "profit_higher" | "profit_lower" => {
                evaluate_guess(company, &guess.guess_type.split('_').last().unwrap(), profit_estimate, company.profit_mil, "B")
            }
            "assets_higher" | "assets_lower" => {
                evaluate_guess(company, &guess.guess_type.split('_').last().unwrap(), assets_estimate, company.asset_mil, "B")
            }
            "employees_higher" | "employees_lower" => {
                evaluate_guess(company, &guess.guess_type.split('_').last().unwrap(), employees_estimate, company.employees, "")
            }
            _ => "Invalid guess".to_string(),
        };

        update_guess_counts(state, &user_id, result.starts_with("Correct"));

        return result;
    }
    "Failed to process the guess".to_string()
}

#[get("/stats")]
fn get_stats(state: &State<AppState>, cookies: &CookieJar<'_>) -> Result<Json<Stats>, &'static str> {
    let user_id = get_user_id(cookies);
    cookies.add(Cookie::new("user_id", user_id.clone()));

    let user_stats = state.user_stats.read().unwrap();
    let stats = user_stats.get(&user_id).cloned().unwrap_or_default();

    Ok(Json(Stats {
        total_games: stats.total_games,
        correct_guesses: stats.correct_guesses,
        incorrect_guesses: stats.incorrect_guesses,
        total_time: stats.total_time,
    }))
}

#[post("/stats", data = "<payload>")]
fn update_stats(state: &State<AppState>, payload: Json<TimePayload>, cookies: &CookieJar<'_>) -> Result<Json<Stats>, &'static str> {
    let user_id = get_user_id(cookies);
    cookies.add(Cookie::new("user_id", user_id.clone()));

    {
        let mut user_stats = state.user_stats.write().unwrap();
        let stats = user_stats.entry(user_id.clone()).or_default();
        stats.total_time += payload.time;
    }

    get_stats(state, cookies)
}

fn get_user_id(cookies: &CookieJar<'_>) -> String {
    cookies.get("user_id").map_or_else(|| Uuid::new_v4().to_string(), |cookie| cookie.value().to_string())
}

#[launch]
fn rocket() -> Rocket<Build> {
    let state = AppState {
        selected_company: RwLock::new(None),
        user_stats: RwLock::new(HashMap::new()),
    };
    rocket::build()
        .manage(state)
        .mount("/", routes![company, submit_guess, get_stats, update_stats])
        .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static")))
}
