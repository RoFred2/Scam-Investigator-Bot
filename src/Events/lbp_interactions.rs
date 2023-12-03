use poise::serenity_prelude as serenity;
use core::time::Duration;
use std::sync::Arc;
use redis::{Commands,AsyncCommands};
use serde_json;
use serenity::{model::application::interaction::Interaction, Context, Message, MessageComponentInteraction, InteractionResponseType};
use crate::Embeds::{cprompt, scam_log};
use crate::{Error, Data};
use serenity::model::prelude::prelude::component::InputTextStyle;
    
async fn on_cancel(ctx : &Context, mci : &MessageComponentInteraction) -> Result<(), Error> {
    mci.defer_ephemeral(&ctx.http).await;

    let og_message = &mci.message;
    let cache_http = &ctx.http;
    let user_id = mci.user.id;
    
    let (cprompt_embed, cprompt_components) = cprompt::CPromptEmbed {
	label: "Are you sure?".to_string(),
	description: "This session will be **DELETED**, so it can't be resumed!".to_string()
    }.cprompt_embed();
    let msg = mci.edit_original_interaction_response(&ctx.http, |b| {
	b.set_embed(cprompt_embed).set_components(cprompt_components)
    }).await?;
    
    let cmp_inter_col = msg.await_component_interaction(&ctx.shard)
	.collect_limit(1 as u32)
	.author_id(user_id)
	.timeout(Duration::from_secs(120));
    // Collected interaction
    let coll_mci : Arc<MessageComponentInteraction> = cmp_inter_col.await.unwrap();
    let cmci_cid = &coll_mci.data.custom_id;

    coll_mci.defer(cache_http).await;
    match cmci_cid.as_str() {
	"cpr_continue" => {
	    mci.delete_original_interaction_response(cache_http).await?;
	    og_message.delete(cache_http).await?;
	},
	"cpr_cancel" => {
	    mci.delete_original_interaction_response(cache_http).await?;
	},
	_ => {}
    }
    Ok(())
}

pub async fn sm_lb(ctx : &Context, mci : &MessageComponentInteraction) -> Result<(),Error> {
    let sm_values = &mci.data.values;
    let selected = sm_values[0].clone();
    
    match selected.as_str() {
	"add_scammer" => {
	    mci.create_interaction_response(&ctx.http, |r| {
		r.kind(InteractionResponseType::Modal)
		  .interaction_response_data(|id| {
		      id
			  .content("")
			  .title("Add Scammer")
			  .custom_id("scammer_modal")
			  .components(|compo| {
			      compo.create_action_row(|ar| {
				  ar
				      .create_input_text(|it| {
					  it
					      .custom_id("scammer_input")
					      .value("000000000000000000")
					      .label("Scammer Id")
					      .required(true)
					      .min_length(18)
					      .max_length(19)
					      .style(InputTextStyle::Short)
				      })
			      })
			  })
			 
		 })
	    }).await.unwrap();
	},
	"add_victim" => {
	    mci.create_interaction_response(&ctx.http, |r| {
		r.kind(InteractionResponseType::Modal)
		  .interaction_response_data(|id| {
		      id
			  .content("")
			  .title("Add Victim")
			  .custom_id("victim_modal")
			  .components(|compo| {
			      compo.create_action_row(|ar| {
				  ar
				      .create_input_text(|it| {
					  it
					      .custom_id("victim_input")
					      .value("000000000000000000")
					      .label("Victim Id")
					      .required(true)
					      .min_length(18)
					      .max_length(19)
					      .style(InputTextStyle::Short)
				      })
			      })
			  })
			 
		 })
	    }).await.unwrap();
	},
	"add_evidence" => {
	    mci.create_interaction_response(&ctx.http, |r| {
		r.kind(InteractionResponseType::Modal)
		  .interaction_response_data(|id| {
		      id
			  .content("")
			  .title("Add Evidence")
			  .custom_id("evidence_modal")
			  .components(|compo| {
			      compo.create_action_row(|ar| {
				  ar
				      .create_input_text(|it| {
					  it
					      .custom_id("evidence_input")
					      .placeholder("https://google.com https://google.com")
					      .label("Evidence")
					      .required(true)
					      .style(InputTextStyle::Paragraph)
				      })
			      })
			  })
			 
		 })
	    }).await.unwrap();
	},
	"add_context" => {
	    mci.create_interaction_response(&ctx.http, |r| {
		r.kind(InteractionResponseType::Modal)
		  .interaction_response_data(|id| {
		      id
			  .content("")
			  .title("Add Context")
			  .custom_id("context_modal")
			  .components(|compo| {
			      compo.create_action_row(|ar| {
				  ar
				      .create_input_text(|it| {
					  it
					      .custom_id("context_input")
					      .label("Context")
					      .placeholder("Time wasting, Scammer did not co-operate.")
					      .required(true)
					      .style(InputTextStyle::Paragraph)
				      })
			      })
			  })
			 
		 })
	    }).await.unwrap();
	},
	_ => {}
    }
    Ok(())
}

pub async fn lbp_interactions(poise_data : &Data,ctx : &Context, interaction : &Interaction) -> Result<(), Error> {
    let mci = interaction.clone().message_component();
    let modal_submit = interaction.clone().modal_submit();
    
    if let Some(modal_submit) = modal_submit {
	let data = &modal_submit.data;
	let c_id = &data.custom_id;
	let action_row = &data.components[0];
	let mut connection = poise_data.redis_client.get_async_connection().await?;
	let channel_id = modal_submit.channel_id;
	let name = channel_id.name(&ctx.cache).await.unwrap();
	let ticket_id = scam_log::get_id(name);
	let mut session_data : scam_log::LogTrigger = serde_json::from_str(&connection.get::<String, String>(ticket_id.clone()).await?).unwrap();
	
	let value = match &action_row.components[0] {
	    serenity::ActionRowComponent::InputText(input_text) => {
		input_text.value.clone()
	    },
	    _ => "".to_string()
	};
	match c_id.as_str() {
	    "scammer_modal" => {
		let formatted_scammer = format!("<@{}>", value);
		session_data.scammer = formatted_scammer.to_string();
	    },
	    "victim_modal" => {
		let formatted_victim = format!("<@{}>", value);
		session_data.victim = formatted_victim.to_string();
	    },
	    "evidence_modal" => {
		// Even if the links are retarded it will still work
		let ns_evidence = value.as_str().replace(" ", "");
		let mut sanitized_evidence = ns_evidence.replace("http", " http");
		sanitized_evidence.remove(0);
		
		let split_evidence : Vec<&str> = sanitized_evidence.split(" ").collect();
		let mut formatted_evidence = String::new();
		for (i,evidence_link) in split_evidence.iter().enumerate() {
		    formatted_evidence.push_str(format!("[{}]({}) | ",i,evidence_link).as_str());
		}
		session_data.evidence = formatted_evidence;
	    }
	    "context_modal" => {
		session_data.context = value;
	    }
	    _ => {}
	}
	let (new_embed, new_components) = session_data.clone().log_trigger();
	let mut og_message = modal_submit.clone().message.unwrap();
	og_message.edit(&ctx.http, |b| {
	    b.set_embed(new_embed)
	}).await?;
	connection.set::<String, String, String>(ticket_id, serde_json::to_string(&session_data)?).await?;
	modal_submit.defer(&ctx.http).await?;
    }
    
    if let Some(mci) = mci {
	let c_id = &mci.data.custom_id;
	match c_id.as_str() {
	    "lb_cancel" => {
		on_cancel(ctx, &mci).await?;
	    },
	    "sm_lb" => {
		sm_lb(ctx, &mci).await?;
	    }
	    _ => {}
	}
    }
    Ok(())
}
