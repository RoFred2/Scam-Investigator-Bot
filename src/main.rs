#![allow(warnings)]
use poise::serenity_prelude as serenity;
use serenity::{GatewayIntents};
use std::env;
use Services::events_service;
// poise types
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let terminal_args : Vec<String> = env::args().collect();
    let targs_len = terminal_args.len();
    if targs_len == 0 {
	println!("Usage: \n- cargo run <Token> OR executable <Token>");
	return;
    }
    // len - 1 because len is displaced forward by 1
    let possible_token = terminal_args[targs_len - 1].clone();
    drop(terminal_args);
    
    let framework_builder : poise::FrameworkBuilder<Data, Error> =
	poise::Framework::builder()
	.options(poise::FrameworkOptions {
	    commands : vec![],
	    event_handler : |ctx, event, framework_context, data| {
		Box::pin(events_service::handle_event(ctx, event))
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
		    Ok(Data {})
		})
	    }
	);
    
    let _ = framework_builder.run().await.unwrap();
}

pub mod Embeds {
    pub mod scam_log;
}

pub mod Events {
    pub mod on_ticket_closed;
}

pub mod Services {
    pub mod events_service;
}
