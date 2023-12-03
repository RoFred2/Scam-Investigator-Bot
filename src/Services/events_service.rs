use poise::{self, Event, serenity_prelude as serenity};
use serenity::{Context};
use crate::{Error,Data, Events::on_ticket_closed,Events::lbp_interactions};

pub async fn handle_event (data : &Data, ctx : &Context, event : &Event<'_>)
    -> Result<(), Error>
{
    match event {
	Event::Ready {data_about_bot} => {
	    println!("[{}] Is online now", data_about_bot.user.name);
	},
	Event::ChannelUpdate {old, new} => {
	   on_ticket_closed::on_ticket_closed(data,ctx,new).await?;
	},
	Event::InteractionCreate {interaction} => {
	   lbp_interactions::lbp_interactions(data,ctx,interaction).await?;
	},
	_ => {}
    }
    Ok(())	    
}
