#![allow(warnings)]
use poise::serenity_prelude as serenity;
use serenity::{GatewayIntents};
use std::{env, fs::File, pin::Pin, io::Write};
use redis::{self, Client};
use Services::events_service;
// poise types
struct Data {
    redis_client : Client,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let terminal_args : Vec<String> = env::args().collect();
    let targs_len = terminal_args.len();
    
    if targs_len <= 2 {
	println!("Usage: \n- cargo run <BotToken> <RedisToken> OR executable <BotToken> <RedisToken>");
	return;
    }

    let redis_tok = terminal_args[2].clone();
    let possible_token = terminal_args[1].clone();
    drop(terminal_args);
    println!("Redis URL: {}\nDiscord Bot Token: {}",redis_tok.as_str(), possible_token.as_str());
    let client = redis::Client::open(redis_tok).unwrap();    
    
    let framework_builder : poise::FrameworkBuilder<Data, Error> =
	poise::Framework::builder()
	.options(poise::FrameworkOptions {
	    commands : vec![],
	    event_handler : |ctx, event, framework_context, data| {
		Box::pin(events_service::handle_event(data,ctx, event))
	    },
	    prefix_options: poise::PrefixFrameworkOptions {
		prefix: Some("sudo".into()),
		edit_tracker: Some(poise::EditTracker::for_timespan(std::time::Duration::from_secs(3600))),
		case_insensitive_commands: true,
		..Default::default()
            },
	    ..Default::default()
	})
	
	.token(possible_token)
	.intents(
	    GatewayIntents::privileged() |
	    GatewayIntents::GUILDS |
	    GatewayIntents::GUILD_MESSAGES
	)
	.setup(
	    |ctx, _ready, framework| {

		Box::pin(async move {
		    poise::builtins::register_globally(
			ctx,&framework.options().commands
		    ).await?;
		    Ok(Data {
			redis_client : client,
		    })
		})
	    }
	);
    
    let _ = framework_builder.run().await.unwrap();
}

pub mod Commands {
}

pub mod Embeds {
    pub mod scam_log;
    pub mod cprompt;
}

pub mod Events {
    pub mod on_ticket_closed;
    pub mod lbp_interactions;
}

pub mod Services {
    pub mod events_service;
}
