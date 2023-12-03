use poise::serenity_prelude as serenity;
use serenity::{Context, Channel};
use redis::AsyncCommands;
use serde_json;
use crate::Embeds::scam_log;
use crate::{Error, Data};
use scam_log::LogTrigger;

pub async fn on_ticket_closed(data : &Data,ctx : &Context, new : &Channel) -> Result<(), Error> {
    let cache = ctx.cache.clone();
    let ch_name = new.id().name(&cache).await.unwrap(); 
    
    if ch_name.contains("closed") {
	let mut redis_connection = data.redis_client.get_async_connection().await?;
	
	let name_split : Vec<&str> = ch_name.split("-").collect();
	let ns_len = name_split.len();

	let ticket_id = name_split[ns_len - 1];
    
	let log_builder = scam_log::LogTrigger {
	    author : cache.current_user().into(),
	    ticket_id : ticket_id.to_string(),
	    scammer : "N/A".to_string(),
	    victim : "N/A".to_string(),
	    evidence : "N/A".to_string(),
	    context : "N/A".to_string()
	};

	let sfed_logb = serde_json::to_string(&log_builder)?;
	redis_connection.set(ticket_id, sfed_logb).await?;
	
	let (embed, components) = log_builder.log_trigger();
	new.id().send_message(ctx.http.clone(),|message| {
	    message
		.set_embed(embed)
		.set_components(components)
	}).await?;    
    }
    Ok(())
}
