use poise::serenity_prelude as serenity;
use serenity::{CreateEmbed, CreateComponents, ButtonStyle};

pub struct CPromptEmbed {
    pub label : String,
    pub description : String
}

impl CPromptEmbed {
    
    pub fn cprompt_embed(self) -> (CreateEmbed, CreateComponents){
	let mut embed = CreateEmbed::default();
	embed
	    .title(self.label)
	    .description(self.description);
	let mut components = CreateComponents::default();
	components
	    .create_action_row(|row| {
		row
		    .create_button(|button| {
			button
			    .custom_id("cpr_continue")
			    .label("Continue")
			    .style(ButtonStyle::Success)
		    })
		    .create_button(|button| {
			button
			    .custom_id("cpr_cancel")
			    .label("Cancel")
			    .style(ButtonStyle::Danger)
		    })
	    });
	(embed, components)
    }
    
}
