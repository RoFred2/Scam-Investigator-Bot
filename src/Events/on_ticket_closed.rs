use poise::serenity_prelude as serenity;
use serenity::{Context, Channel};

use crate::Embeds::scam_log;
use scam_log::LogTrigger;

pub async fn on_ticket_closed(ctx : &Context, new : &Channel) {
    println!("[TICKET CLOSED - INIT INTERFACE]");
    let cache = ctx.cache.clone();
    let ch_name = match new.id().name(&cache).await {
	Some(name) => {
	    println!("-    Fetched name");
	    name
	},
	None => {
	    println!("-    Failed to fetch name");
	    return
	}
    };
    
    if !ch_name.contains("closed") {
	println!("-    Ticket doesn't contain 'closed'");
	return;
    }
    let name_split : Vec<&str> = ch_name.split("-").collect();
    let ns_len = name_split.len();
    let ticket_id = name_split[ns_len - 1];
    println!("-    Ticket ID: {}",ticket_id);

    let log_builder = scam_log::LogTrigger {
	author : cache.current_user().into(),
	ticket_id : ticket_id.to_string(),
	ses_id : "1".to_string(), // Temporary until DB implementation
	scammer : "N/A".to_string(),
	victim : "N/A".to_string(),
	evidence : "N/A".to_string(),
	context : "N/A".to_string()
    };
    println!("-    Sending message..");
    let (embed, components) = log_builder.log_trigger();
    new.id().send_message(ctx.http.clone(),|message| {
	message
	    .set_embed(embed)
	    .set_components(components)
    }).await.map_err(|err| println!("Failed to send build interface : {}", err.to_string()));
}
