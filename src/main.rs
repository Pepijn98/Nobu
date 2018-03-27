#[macro_use]
extern crate serenity;
extern crate chrono;
extern crate rand;
extern crate kitsu;
extern crate time;
extern crate cleverbot_io;
extern crate env_logger;
extern crate kankyo;
extern crate colored;
extern crate psutil;
extern crate schedule_recv;
extern crate serde_json;
extern crate typemap;
extern crate reqwest;
extern crate tokio_core;

mod commands;
mod utils;

use serenity::prelude::*;
use serenity::client::bridge::gateway::ShardManager;
use serenity::model::{prelude::Game, user::OnlineStatus, guild::*, gateway::Ready};
use serenity::framework::standard::{StandardFramework, HelpBehaviour, help_commands};
use std::collections::HashMap;
use serenity::prelude::Mutex;
use std::sync::Arc;
use std::time::Duration;
use std::{env, thread};
use rand::Rng;
use typemap::Key;

struct ShardManagerContainer;

impl Key for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl Key for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;

impl EventHandler for Handler {
	fn ready(&self, ctx: Context, ready: Ready) {
		/* It seems like getting all members on ready is too much with ~8k users
		let mut user_ids: Vec<UserId> = Vec::new();

		for guild in &ready.guilds {
			let all = guild.id().members::<UserId>(None, None).unwrap();
			for mem in all {
				user_ids.push(mem.user.read().id);
			}
		}

		user_ids.sort();
		user_ids.dedup(); */

		if let Some(shard) = ready.shard {
			utils::logger::info(format!(
				"I'm back master! This is shard {}/{} which has {} guilds",
				shard[0],
				shard[shard.len() - 1],
				ready.guilds.len())
			);
		}

		let games = vec![
			"with Senpai",
			"with my master",
			"visual novels",
			"type `n:help`",
			"prefix: `n:`",
			"with your feelings"
		];
		
		thread::spawn(move || {
				loop {
					let game = rand::thread_rng().choose(&games);
					ctx.set_presence(Some(Game::playing(game.unwrap())), OnlineStatus::Online);
					thread::sleep(Duration::from_secs(1800));
				}
		});
	}

	fn guild_create(&self, _: Context, guild: Guild, boolean: bool) {
		if boolean == true {
			utils::logger::info(format!("Joined guild {} - ({})", guild.name, guild.id));
			utils::utils::exec_join_webhook(guild);
		}
	}

	fn guild_delete(&self, _: Context, guild: PartialGuild, _: Option<Arc<RwLock<Guild>>>) {
		utils::logger::error(format!("Left guild {} - ({})", guild.name, guild.id));
		utils::utils::exec_leave_webhook(guild);
	}
}

fn main() {
	kankyo::load().expect("Failed to load .env file");
	env_logger::init().expect("Failed to initialize env_logger");

	let mut token = String::new();
	let mut prefix = String::new();

	let release = env::var("RELEASE").expect("Expected a release type");
	
	if &release == "dev" {
		token = env::var("DISCORD_DEV_TOKEN").expect("Expected a dev token");
		prefix = env::var("DEV_PREFIX").expect("Expected a dev prefix");
	} else if &release == "prod" {
		token = env::var("DISCORD_TOKEN").expect("Expected a production token");
		prefix = env::var("PREFIX").expect("Expected a production prefix");
	}

	let mut client = Client::new(&token, Handler).expect("Error creating the client");

	{
		let mut data = client.data.lock();
		data.insert::<CommandCounter>(HashMap::default());
		data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
	}

	client.with_framework(
		StandardFramework::new()
			.configure(|c| c
			.on_mention(true)
			.allow_dm(false)
			.prefix(&prefix)
			.delimiters(vec!["| ", "|", " |", " | "]))
		.after(|_, msg, _, _error| {
			if let Some(guild) = msg.guild() {
				let guild = guild.read();
				utils::logger::command(&guild.name, &msg.author.name, &msg.content);
			} else {
				utils::logger::command_dm("DM", &msg.author.name, &msg.content);
			}
		})
		.complex_bucket("main", 5, 0, 2, |_, _, _, user_id| {
			user_id.0.to_string() != env::var("OWNER").unwrap()
		})
		.customised_help(help_commands::with_embeds, |c| {
			c.individual_command_tip("For more info on a specific command just pass the command as an argument.")
			.suggestion_text("I think you meant this command: `{}`")
			.lacking_permissions(HelpBehaviour::Hide)
			.lacking_role(HelpBehaviour::Hide)
			.wrong_channel(HelpBehaviour::Strike)
		})
		.group("General", |g| g
			.command("userinfo", |c| c
				.cmd(commands::general::userinfo)
				.bucket("main")
				.known_as("ui")
				.desc("Displays user info"))
			.command("ping", |c| c
				.cmd(commands::general::ping)
				.bucket("main")
				.desc("Check if the bot works"))
			.command("cleverbot", |c| c
				.cmd(commands::general::cleverbot)
				.bucket("main")
				.known_as("talk")
				.desc("Have a nice converstion with Maika"))
			.command("stats", |c| c
				.cmd(commands::general::stats)
				.bucket("main")
				.desc("Show stats about Maika"))
			.command("invite", |c| c
				.cmd(commands::general::invite)
				.bucket("main")
				.known_as("inv")
				.desc("Get Maika's invite link")))
		.group("Random", |g| g
			.command("choose", |c| c
				.cmd(commands::random::choose)
				.bucket("main")
				.desc("Choose between options"))
			.command("coinflip", |c| c
				.cmd(commands::random::coinflip)
				.bucket("main")
				.known_as("cf")
				.desc("Flip a coin"))
			.command("8ball", |c| c
				.cmd(commands::random::magic_eight_ball)
				.bucket("main")
				.desc("8ball"))
			.command("roll", |c| c
				.cmd(commands::random::roll)
				.bucket("main")
				.desc("Roll a random number"))
			.command("roulette", |c| c
				.cmd(commands::random::roulette)
				.bucket("main")
				.desc("Roulette")))
		.group("Otaku", |g| g
			.command("anime", |c| c
				.cmd(commands::otaku::anime)
				.bucket("main")
				.desc("Shows info about an anime"))
			.command("manga", |c| c
				.cmd(commands::otaku::manga)
				.bucket("main")
				.desc("Shows info about a manga")))
	);

	if let Err(err) = client.start_autosharded() {
		utils::logger::error(format!("Client error:\n{:?}", err));
	}
}
