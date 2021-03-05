extern crate binance;

// use std::borrow::Borrow;
use std::{thread, time};

use colored::*;
use configparser::ini::Ini;

use binance::{api::*, model::SymbolPrice};
use binance::market::*;
use binance::account::*;

fn main() {

    let mut config = Ini::new();
    let map = config.load("config.toml");
    match map {
        Ok(_) => {
            let api_key = Some(config.get("keys", "api_key").unwrap().into());
            let secret_key = Some(config.get("keys", "secret_key").unwrap().into());
            // let account: Account = Binance::new(api_key.clone(), secret_key.clone());
            let market: Market = Binance::new(api_key, secret_key);
            
            // buy_symbol_with_btc(market, account);

            let symbols = vec!["SANDUSDT", "SFPUSDT", "FIROUSDT", "DODOUSDT"];
            
            let mut epoch = 0;
            // let now = time::Instant::now();
            let one_second = time::Duration::from_millis(2000);
            loop {
                whale_scan(&market, &symbols, epoch);
                thread::sleep(one_second);
                epoch += 1; // increase
            }
        },
        Err(_) => println!("{}","can't load config file.".red())
    }
}

fn whale_scan(market: &Market, symbols: &Vec<&str>, epoch: i32)
{
    println!("{}",format!("\n---------------[ EPOCH #{} ]----------------\n", epoch).yellow());   
    //
    // Loop through all symbols
    //
    for symbol in symbols {
        // begin
        match &market.get_average_price(*symbol) {
            Ok(answer) => { 
                //
                // indicate we are riding whale or not 
                let mut is_whale_riding = false;
                //
                // Check Average price first
                let _average = answer.price;
                //
                // compute diff
                let _diff = compute_change(&market, symbol, _average);
                //
                // processing "changes" when we already know it's "Rise" of "Fall"
                //
                decision_making(_diff,  &mut is_whale_riding, symbol);
                //
                // end line
                println!();
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn buy_symbol_with_btc<S>(market: Market, account: Account) 
where S: Into<String> 
{
    println!("Which symbol to buy ? ");

    let mut symbol:String = String::new();
    std::io::stdin()
        .read_line(&mut symbol)
        .ok()
        .expect("invalid symbol !");

    // convert to String to borrow later
    let _symbol:String = symbol.into();

    // Latest price for ONE symbol
    match market.get_price(&_symbol) {
        Ok(answer) => {
            println!("\n- {}: {}", answer.symbol, answer.price);
            let current_price = answer.price;

            // get all BTC 1st 
            match account.get_balance("BTC") {
                Ok(answer) => {
                    println!("- BTC free: {}", answer.free);
                    // "balances": [
                    // {
                    //     "asset": "BTC",
                    //     "free": "4723846.89208129",
                    //     "locked": "0.00000000"
                    // },
                    let available_btc:f64 = answer.free.parse().unwrap();
                    let qty = &available_btc / &current_price;
                    //
                    // we convert all current BTC into the next coin:
                    //

                    println!("- market_buy {} {}", qty ,_symbol);

                    // buy all with btc 
                    match account.market_buy(&_symbol, qty) {
                        Ok(answer) => {
                            println!("- success => {:?}\n", answer)
                        },
                        Err(e) => println!("- ERROR: \n{:?}", e),
                    }
                },
                Err(e) => println!("Error: {:?}", e),
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\n");
}

fn compute_change(market: &Market, symbol: &&str, average:f64) -> f64
{
    //
    // Compare to latest price 
    //
    let mut _diff = 0.0;
    fn detect_changes(changes: f64) -> String {
        return String::from(if changes > 1.0 {"Gain"} else {"Loss"});
    }
    match market.get_price(*symbol) {
        Ok(answer) => {
            // calculate stuffs 
            let _changes = answer.price / average;
            _diff = (_changes-1.0) * 100.0;
            // log
            let log = format!("{:<10}: average = {:.2} | {}: {:.2}%", symbol, average, detect_changes(_changes), _diff);
            print!("{}", if _diff > 0.0 {log.green()} else {log.red()});
        },
        Err(e) => println!("Error: {}", e),
    }
    return _diff;  // return
}

fn decision_making(_diff:f64, is_whale_riding: &mut bool, symbol: &&str){
    //
    // Define BUY RATIO here
    //
    let ratio_whale_pump:f64 = 2.0; // percent

    if _diff > ratio_whale_pump {
        //
        // BUY
        //
        print!("{}", format!("=> WHALE DETECTED => BUY XXX {}", symbol).white().bold());
        *is_whale_riding = true;

    } 
    else {
        if *is_whale_riding {
            //
            // SELL
            //
            print!("{}",format!(" => WHALE DUMP => SELL ALL {}", symbol).red());
        } 
        else {
            //
            // DOING NOTHING 
            //
        }
    }
}