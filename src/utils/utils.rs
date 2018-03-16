use serenity::http;
use serenity::model::{channel::*, guild::*};
use serenity::utils::Colour;
use serenity::CACHE;
use std::env;
use std::collections::HashMap;

pub fn exec_join_webhook(guild: Guild) {
	let webhook_id = &env::var("JoinLeaveWebhookID").unwrap();
	let webhook_token = &env::var("JoinLeaveWebhookToken").unwrap();
	let members: String = guild.member_count.to_string();
	let channels: String = guild.channels.len().to_string();
	let webhook = http::get_webhook_with_token(webhook_id.parse::<u64>().unwrap(), webhook_token).expect("valid webhook");
	let guild_icon = guild.icon_url().unwrap_or_else(|| String::from(""));

	let join = Embed::fake(|e| e
		.title("Joined Guild:")
		.description(format!("{} ({})", guild.name, guild.id))
		.thumbnail(&guild_icon)
		.colour(Colour::from_rgb(246, 219, 216))
		.field("Members", &members, true)
		.field("Channels", &channels, true));

	let cache = CACHE.read();

	let _ = webhook.execute(false, |w| w
		.username(&cache.user.name)
		.avatar_url(&cache.user.face())
		.embeds(vec![join]));
}

pub fn exec_leave_webhook(guild: PartialGuild) {
	let webhook_id = &env::var("JoinLeaveWebhookID").unwrap();
	let webhook_token = &env::var("JoinLeaveWebhookToken").unwrap();
	let members = guild.members(Some(0), Some(0)).unwrap_or_else(|_| vec![]).len().to_string(); // Errors when members() is empty even tho it says both are optional...
	let channels = guild.channels().unwrap_or_else(|_| HashMap::new()).len().to_string();
	let webhook = http::get_webhook_with_token(webhook_id.parse::<u64>().unwrap(), webhook_token).expect("valid webhook");
	let guild_icon = guild.icon_url().unwrap_or_else(|| String::from(""));

	let leave = Embed::fake(|e| e
		.title("Left Guild:")
		.description(format!("{} ({})", guild.name, guild.id))
		.thumbnail(&guild_icon)
		.colour(14038325)
		.field("Members", &members, true)
		.field("Channels", &channels, true));

	let cache = CACHE.read();

	let _ = webhook.execute(false, |w| w
		.username(&cache.user.name)
		.avatar_url(&cache.user.face())
		.embeds(vec![leave]));
}