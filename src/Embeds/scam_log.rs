use poise::serenity_prelude as serenity;
use serenity::{User, CreateEmbed, CreateComponents, ButtonStyle, ReactionType};

// Acts as a param builder
pub struct LogTrigger {
    pub author : User,
    pub ticket_id : String,
    pub ses_id : String,
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
	    .footer(|f| {
		f
		    .text(format!("Session ID: {}", self.ses_id))
	    })
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
					    .default_selection(true)
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
