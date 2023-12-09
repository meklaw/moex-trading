use std::env;
use teloxide::Bot;
use teloxide::prelude::{ChatId, Requester};
use tokio::runtime;
use crate::models::algopack::candle_trade_stats::CandleTradeStats;
use crate::traits::trade::Trade;

pub struct TestTradingSystem {
    _prev_candel: Option<CandleTradeStats>,
    financial_result: f64,
    is_open: bool,
    position_price_open: f64,
    bot: Bot,
    chanel_id: i64,
}

impl Trade for TestTradingSystem {
    fn new() -> Self {
        Self { _prev_candel: None, financial_result: 0.0, is_open: false, position_price_open: 0.0, bot: Bot::from_env(), chanel_id: env::var("CHANEL_ID").expect("You need to specify the CHANEL_ID in the telegram").parse().unwrap() }
    }

    fn trade(&mut self, record: CandleTradeStats) {
        let open_position_predict = record.pr_close > record.pr_open;
        let close_position_predict = self.is_open;


        if !self.is_open && open_position_predict {
            self.position_price_open = record.pr_close.unwrap();
            self.is_open = true;
            self.send_open_signal(record.secid, record.pr_close.unwrap());
        } else if self.is_open && close_position_predict {
            let position_result = record.pr_close.unwrap() - self.position_price_open;
            self.financial_result += position_result;
            self.send_close_signal(record.secid, record.pr_close.unwrap());
            self.is_open = false;
        }
    }
}

impl TestTradingSystem {
    fn send_open_signal(&self, secid: String, price: f64) {
        let formatted_string = format!(r#"
open long
{} by {}
"#, secid, price);

        self.send_message(formatted_string);
    }
    fn send_close_signal(&self, secid: String, price: f64) {
        let formatted_string = format!(r#"
close long
{} by {}
"#, secid, price);
        self.send_message(formatted_string);
    }

    fn send_message(&self, message: String) {
        runtime::Runtime::new().unwrap().block_on(async {
            self.bot
                .send_message(ChatId(self.chanel_id), message)
                .await
                .expect("Error by sending message");
        });
    }
    pub fn get_financial_result(&self) -> f64 {
        return self.financial_result;
    }
}