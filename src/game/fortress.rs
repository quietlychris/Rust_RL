use std::cmp::Ordering;
use crate::board::smart_board;
use crate::agent::agent::AgentType;
use crate::agent::agent_trait::Agent;

pub struct Game {
    rounds: u8,
    res: (u32, u32, u32),
    last_result: i32,
    agent1: AgentType,
    agent2: AgentType,
}

impl Game {
    pub fn new(rounds_per_game: u8, game_type: u8) -> Result<Self, String> {
      // first digit encondes type of first agent, second digit the type of the second agent
      let first_agent = AgentType::create_agent(rounds_per_game, game_type/10, true)?; //first_player = true
      let second_agent = AgentType::create_agent(rounds_per_game, game_type%10, false)?; // first_player = false

      Ok(Game {
          rounds: rounds_per_game,
          res: (0, 0, 0),
          last_result: 42, //init value shouldn't be used
          agent1: first_agent,
          agent2: second_agent,
      })
    }

    fn update_results(&mut self, first_player_fields: u8, second_player_fields: u8) {
      match first_player_fields.cmp(&second_player_fields) {
        Ordering::Greater => {
          self.res.0 += 1;
          self.last_result = 1;   
        },
        Ordering::Less => {
          self.res.2 += 1;
          self.last_result = -1;
        },
        Ordering::Equal => {
          self.res.1 += 1;
          self.last_result = 0;
        },
      }
    }


    pub fn get_results(&self) -> (u32, u32, u32) {
        self.res
    }

    pub fn get_agent_ids(&self) -> (String, String) {
        (self.agent1.get_id(), self.agent2.get_id())
    }

    pub fn train(&mut self, num_games: u64) {
        self.play_games(num_games, true);
    }

    pub fn bench(&mut self, num_games: u64) -> (u32, u32, u32) {
        self.agent1.set_exploration_rate(0.).unwrap(); // exploration rate is in [0,1], so ignore error possibility
        self.agent2.set_exploration_rate(0.).unwrap();
        self.play_games(num_games, false)
    }

    //fn get_reward(&mut self) -> f32 {
    //  42.
    //}

    fn play_games(&mut self, num_games: u64, train: bool) -> (u32, u32, u32) {
        self.res = (0, 0, 0);
        let exploration_rate1 = self.agent1.get_exploration_rate();
        let exploration_rate2 = self.agent2.get_exploration_rate();
        let mut board: smart_board::Board;
        let sub_epoch: u64 = (num_games / 10) as u64;

        for game in 0..num_games {
            board = smart_board::Board::get_board(self.rounds);
            if (game % sub_epoch) == 0  && train{
              let sub_epoch_nr: f32 = (game / sub_epoch) as f32;
              let (new_exploration_rate1, new_exploration_rate2) = ((exploration_rate1 * (10. - sub_epoch_nr) / 10.), (exploration_rate2 * (10. - sub_epoch_nr) / 10.));
              self.agent1.set_exploration_rate(new_exploration_rate1).unwrap();
              self.agent2.set_exploration_rate(new_exploration_rate2).unwrap();
              println!("New exploration rates: {}, {}", new_exploration_rate1, new_exploration_rate2);
            }

            for _round in 0..board.get_total_rounds() {

              //TODO what if no move possible? 
              // parallelize

                board.try_move(self.agent1.get_move(&board));
                board.try_move(self.agent2.get_move(&board));
            }

            let game_res = board.eval(); // umstellen auf 1-hot encoded
            self.update_results(game_res.0, game_res.1);
            if train {
                self.agent1.finish_round(self.last_result);
                self.agent2.finish_round(self.last_result);
            }
        }
        self.res
    }
}
