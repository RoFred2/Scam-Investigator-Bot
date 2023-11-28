use poise::{self, Event, serenity_prelude as serenity};
use serenity::{Context};
use crate::Error;

pub async fn handle_event (ctx : &Context, event : &Event<'_>)
    -> Result<(), Error>
{
    match event {
	Event::Ready {data_about_bot} => {
	    println!("[{}] Is online now", data_about_bot.user.name);
	},
	_ => {}
    };
    Ok(())	    
}
