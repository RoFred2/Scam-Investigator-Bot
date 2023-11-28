use poise::{self, Event, serenity_prelude as serenity};
use serenity::{Context};
use crate::{Error, Events::on_ticket_closed};

pub async fn handle_event (ctx : &Context, event : &Event<'_>)
    -> Result<(), Error>
{
    match event {
	Event::Ready {data_about_bot} => {
	    println!("[{}] Is online now", data_about_bot.user.name);
	},
	Event::ChannelUpdate {old, new} => {
	   on_ticket_closed::on_ticket_closed(ctx, new).await;
	},
	_ => {}
    };
    Ok(())	    
}
