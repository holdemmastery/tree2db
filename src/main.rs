mod types;
mod utils;
#[macro_use]
extern crate dotenvy_macro;
use mongodb::{bson::doc, options::ClientOptions, Client };
use crate::types::{Position, Street, Player, MoveOption, Action, OptionEVPair, Problem};
use crate::utils::current_time_millis;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse_async(dotenv!("mongo_uri")).await?;
    let client = Client::with_options(client_options)?;

    client.database("db").run_command(doc! { "ping": 1 }, None).await?;
    println!("Pinged your deployment.");

    let problem = Problem {
        time_created: current_time_millis(),
        hero_position: Position::BB, 
        villain_position: Position::UTG,
        hole_cards: "2s7s".to_string(),
        board: "Ah2h8sTdTc".to_string(),
        problem_elo: 1000,
        action_history: vec![
            Action { street: Street::PreFlop, actor: Player::Villain, option: MoveOption::Bet175 },
            Action { street: Street::PreFlop, actor: Player::Hero, option: MoveOption::Call },
            Action { street: Street::Flop, actor: Player::Villain, option: MoveOption::Bet100 },
            Action { street: Street::Flop, actor: Player::Hero, option: MoveOption::Call },
            Action { street: Street::Turn, actor: Player::Villain, option: MoveOption::Check },
            Action { street: Street::Turn, actor: Player::Hero, option: MoveOption::Check },
            Action { street: Street::River, actor: Player::Villain, option: MoveOption::Bet50 },
        ],
        river_option_evs: vec![
            OptionEVPair { option: MoveOption::Fold, ev: 0_f32 },
            OptionEVPair { option: MoveOption::Call, ev: -4.8141_f32 },
        ],
    };
    
    let problems = client.database("db").collection::<Problem>("problems");

    let insert_result = problems.insert_one(problem, None).await?;
    println!("doc added. id: {:?}", insert_result.inserted_id);

    Ok(())
}
