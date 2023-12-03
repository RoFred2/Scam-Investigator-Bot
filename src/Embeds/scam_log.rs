use poise::serenity_prelude as serenity;
use serenity::{User, CreateEmbed, CreateComponents, InteractionResponseType, ButtonStyle, ReactionType, builder::CreateInteractionResponseData};
use serde::{Serialize, Deserialize};
use serenity::model::prelude::prelude::component::InputTextStyle;
// Acts as a param builder
#[derive(Clone,Serialize, Deserialize)]
pub struct LogTrigger {
    pub author : User,
    pub ticket_id : String,
    pub scammer : String,
    pub victim : String,
    pub evidence : String,
    pub context : String,
}

impl LogTrigger {
    /// Responsible for providing an embed for building the scam log. This will usually be used when a ticket is closed or for whatever prelude command
    pub fn log_trigger(self) -> (CreateEmbed, CreateComponents) {
	// I God damn hate this builder pattern, it looks so ugly
	let mut embed = CreateEmbed::default();
	embed
	    .title(format!(
		"Ticket **__#{}__** [State: CLOSED]"
		, self.ticket_id.as_str()
	    ))
	    .color((47, 49, 54))
	    .field("🔒 Scammer",self.scammer,true) // The 🔒 symbol means that this field is mandatory to fill out!
	    .field("🔒 Victim", self.victim, true)
	    .field("🔒 Evidence", self.evidence, true)
	    .field("Context", self.context, true)
	    .author(|a| {
		let bot_name = self.author.clone().name;
		let bot_face = self.author.face();
		a
		    .name(bot_name)
		    .icon_url(bot_face)
	    });
	let mut components = CreateComponents::default();
	components
	    .create_action_row(|row| {
		row
		    .create_select_menu(|menu| {
			menu
			    .custom_id("sm_lb")
			    .options(|options| {
				options
				    .create_option(|opt| {
					opt
					    .label("Add Scammer")
					    .value("add_scammer")
					    .emoji(ReactionType::Unicode("🕵️".to_string()))
				    })
				    .create_option(|opt| {
					opt
					    .label("Add Victim")
					    .value("add_victim")
					    .emoji(ReactionType::Unicode("👨‍🦯".to_string()))
				    })
				    .create_option(|opt| {
					opt
					    .label("Add Evidence")
					    .value("add_evidence")
					    .emoji(ReactionType::Unicode("🗒️".to_string()))
				    })
				    .create_option(|opt| {
					opt
					    .label("Add Context")
					    .value("add_context")
					    .emoji(ReactionType::Unicode("🔍".to_string()))
				    })
			    })
		    })

	    })
	    .create_action_row(|row| {
		row
		    .create_button(|button| {
			button
			    .style(ButtonStyle::Success)
			    .label("Help")
			    .emoji(ReactionType::Unicode("❓".to_string()))
			    .custom_id("lb_help")
		    })
		    .create_button(|button| {
			button
			    .style(ButtonStyle::Secondary)
			    .label("Cancel")
			    .custom_id("lb_cancel")
		    })
	    });
	
	(embed, components)
    }    
}

pub fn get_scammer_modal<'a>() -> CreateInteractionResponseData<'a> {
    let mut ird = CreateInteractionResponseData::default();
    ird.
    	title("Please provide a scammer ID")
	.components(|c| {
	    c.create_action_row(|ar| {
		ar.create_input_text(|it| {
		    it.custom_id("add_scammer_modal")
			.style(InputTextStyle::Short)
			.label("Scammer ID")
			.placeholder("000000000000000000")
			.min_length(18)
			.max_length(19)
			.value("add_scammer_value")
			.required(true)
		})
	    })
	});
    ird
}

pub fn get_id(name : String) -> String {
    let split_name : Vec<&str> = name.split("-").collect();
    split_name[1].to_string()
}
