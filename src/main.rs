use rocket::{get, routes, launch, Build, Rocket, form::{Form, FromForm}, State};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rand::seq::SliceRandom;
use std::{sync::RwLock, fs::File};

#[macro_use] extern crate rocket;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Company {
    name: String,
    rank: u32,
    industry: String,
    sector: String,
    headquarters_state: String,
    headquarters_city: String,
    market_value_mil: u64,
    revenue_mil: u64,
    profit_mil: u64,
    asset_mil: u64,
    employees: u64,
    description: String,
}

#[derive(Debug)]
struct AppState {
    selected_company: RwLock<Option<Company>>,
}

#[derive(FromForm)]
struct Guess {
    guess_type: String,
}

fn read_csv() -> Result<Vec<Company>, std::io::Error> {
    let file = File::open("data/f500data.csv")?;
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

#[post("/submit_guess", data = "<guess>")]
fn submit_guess(guess: Form<Guess>, state: &State<AppState>) -> String {
    let guess = guess.into_inner();
    let market_cap_estimate = 50_000; // value in millions
    let revenue_estimate = 30_000; // value in millions
    let profit_estimate = 10_000; // value in millions
    let assets_estimate = 25_000; // value in millions
    let employees_estimate = 30_000; // raw value

    let selected_company = state.selected_company.read().unwrap();
    if let Some(ref company) = *selected_company {
        let formatted_market_cap_billion = format!("${:.1}B", company.market_value_mil as f64 / 1_000.0);
        let formatted_revenue_billion = format!("${:.1}B", company.revenue_mil as f64 / 1_000.0);
        let formatted_profit_billion = format!("${:.1}B", company.profit_mil as f64 / 1_000.0);
        let formatted_assets_billion = format!("${:.1}B", company.asset_mil as f64 / 1_000.0);
        let formatted_employees = format!("{}", company.employees);

        let result = match guess.guess_type.as_str() {
            "market_cap_higher" => {
                if company.market_value_mil > market_cap_estimate {
                    format!("Correct! The actual market cap of {} is {} which is higher than $40.0B", company.name, formatted_market_cap_billion)
                } else {
                    format!("Incorrect! The actual market cap of {} is {} which is lower than $40.0B", company.name, formatted_market_cap_billion)
                }
            }
            "market_cap_lower" => {
                if company.market_value_mil < market_cap_estimate {
                    format!("Correct! The actual market cap of {} is {} which is lower than $40.0B", company.name, formatted_market_cap_billion)
                } else {
                    format!("Incorrect! The actual market cap of {} is {} which is higher than $40.0B", company.name, formatted_market_cap_billion)
                }
            }
            "revenue_higher" => {
                if company.revenue_mil > revenue_estimate {
                    format!("Correct! The actual revenue of {} is {} which is higher than $30.0B", company.name, formatted_revenue_billion)
                } else {
                    format!("Incorrect! The actual revenue of {} is {} which is lower than $30.0B", company.name, formatted_revenue_billion)
                }
            }
            "revenue_lower" => {
                if company.revenue_mil < revenue_estimate {
                    format!("Correct! The actual revenue of {} is {} which is lower than $30.0B", company.name, formatted_revenue_billion)
                } else {
                    format!("Incorrect! The actual revenue of {} is {} which is higher than $30.0B", company.name, formatted_revenue_billion)
                }
            }

            "profit_higher" => {
                if company.profit_mil > profit_estimate {
                    format!("Correct! The actual profit of {} is {} which is higher than $10.0B", company.name, formatted_profit_billion)
                } else {
                    format!("Incorrect! The actual profit of {} is {} which is lower than $10.0B", company.name, formatted_profit_billion)
                }
            }
            "profit_lower" => {
                if company.profit_mil < profit_estimate {
                    format!("Correct! The actual profit of {} is {} which is lower than $10.0B", company.name, formatted_profit_billion)
                } else {
                    format!("Incorrect! The actual profit of {} is {} which is higher than $10.0B", company.name, formatted_profit_billion)
                }
            }
            "assets_higher" => {
                if company.asset_mil > assets_estimate {
                    format!("Correct! The actual assets of {} are {} which is higher than $25.0B", company.name, formatted_assets_billion)
                } else {
                    format!("Incorrect! The actual assets of {} are {} which is lower than $25.0B", company.name, formatted_assets_billion)
                }
            }
            "assets_lower" => {
                if company.asset_mil < assets_estimate {
                    format!("Correct! The actual assets of {} are {} which is lower than $25.0B", company.name, formatted_assets_billion)
                } else {
                    format!("Incorrect! The actual assets of {} are {} which is higher than $25.0B", company.name, formatted_assets_billion)
                }
            }
            "employees_higher" => {
                if company.employees > employees_estimate {
                    format!("Correct! The actual employees of {} are {} which is higher than 30,000", company.name, formatted_employees)
                } else {
                    format!("Incorrect! The actual employees of {} are {} which is lower than 30,000", company.name, formatted_employees)
                }
            }
            "employees_lower" => {
                if company.employees < employees_estimate {
                    format!("Correct! The actual employees of {} are {} which is lower than 30,000", company.name, formatted_employees)
                } else {
                    format!("Incorrect! The actual employees of {} are {} which is higher than 30,000", company.name, formatted_employees)
                }
            }
            _ => "Invalid guess".to_string(),
        };
        return result;
    }
    "Failed to process the guess".to_string()
}

#[launch]
fn rocket() -> Rocket<Build> {
    let state = AppState {
        selected_company: RwLock::new(None),
    };
    rocket::build()
        .manage(state)
        .mount("/", routes![company, submit_guess])
        .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static")))
}
