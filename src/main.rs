mod types;

#[macro_use]
extern crate dotenvy_macro;

use mongodb::{bson::doc, options::ClientOptions, Client };
use crate::types::{Position, Street, Player, MoveOption, Action, OptionEVPair, Problem};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse_async(dotenv!("mongo_uri")).await?;
    let client = Client::with_options(client_options)?;

    // Send a ping to confirm a successful connection
    client.database("db").run_command(doc! { "ping": 1 }, None).await?;
    println!("Pinged your deployment.");

    let problem = Problem {
        time_created: 123456790,
        hero_position: Position::BB, 
        villain_position: Position::UTG,
        hole_cards: "3dAc".to_string(),
        board: "Th9h2h2c3h".to_string(),
        action_history: vec![
            Action { street: Street::PreFlop, actor: Player::Villain, option: MoveOption::Bet175 },
            Action { street: Street::PreFlop, actor: Player::Hero, option: MoveOption::Call },
            Action { street: Street::Flop, actor: Player::Villain, option: MoveOption::Bet175 },
            Action { street: Street::Flop, actor: Player::Hero, option: MoveOption::Call },
            Action { street: Street::Turn, actor: Player::Villain, option: MoveOption::Bet175 },
            Action { street: Street::Turn, actor: Player::Hero, option: MoveOption::Call },
            Action { street: Street::Turn, actor: Player::Villain, option: MoveOption::Bet175 },
        ],
        river_option_evs: vec![
            OptionEVPair { option: MoveOption::Fold, ev: -1.34834_f32 },
            OptionEVPair { option: MoveOption::Call, ev: 21.99349_f32 },
        ],
    };
    
    let problems = client.database("db").collection::<Problem>("problems");

    let insert_result = problems.insert_one(problem, None).await?;
    println!("doc added. id: {:?}", insert_result.inserted_id);

    Ok(())
}
