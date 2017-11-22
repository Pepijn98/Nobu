extern crate serenity;
extern crate chrono;
extern crate rand;
extern crate kitsu_io;
extern crate time;
extern crate cleverbot_io;
extern crate env_logger;
extern crate kankyo;
extern crate colored;

mod commands;
mod utils;

use serenity::prelude::*;
use serenity::model::*;
use serenity::client::{Client, Context};
use serenity::framework::standard::{StandardFramework, help_commands};
use std::sync::{Arc};
use std::env;

struct Handler;

impl EventHandler for Handler {
  fn ready(&self, ctx: Context, ready: Ready) {
    utils::logger::info(format!("{} is connected. Serving {} servers", ready.user.name, ready.guilds.len()));
    ctx.set_presence(Some(Game::playing("with rust!")), OnlineStatus::Online);
  }

  fn guild_create(&self, _: Context, guild: Guild, boolean: bool) {
    if boolean == true {
      utils::logger::info(format!("Joined guild {} - ({})", guild.name, guild.id));
    }
  }

  fn guild_delete(&self, _: Context, guild: PartialGuild, _: Option<Arc<RwLock<Guild>>>) {
    utils::logger::error(format!("Left guild {} - ({})", guild.name, guild.id));
  }
}

fn main() {
  kankyo::load().expect("Failed to load .env file");
  env_logger::init().expect("Failed to initialize env_logger");

  let token = env::var("DISCORD_TOKEN").unwrap();
  let prefix = env::var("PREFIX").unwrap();
  let mut client = Client::new(&token, Handler).unwrap();

  client.with_framework(StandardFramework::new()
    .configure(|c| c
      .on_mention(true)
      .allow_dm(false)
      .prefix(&prefix))
    .help(help_commands::with_embeds)
    .before(|_ctx, msg, _cmd_name| {
      if let Some(guild) = msg.guild() {
        let guild = guild.read();
        utils::logger::command(&guild.name, &msg.author.name, &msg.content);
      } else {
        utils::logger::command_dm("DM", &msg.author.name, &msg.content);
      }
      true
    }).complex_bucket("main", 5, 0, 2, |_, _, _, user_id| {
      user_id.0.to_string() != env::var("OWNER").unwrap()
    })
    .group("General", |g| g
      .command("userinfo", |c| c
        .exec(commands::general::userinfo)
        .bucket("main")
        .known_as("ui")
        .desc("Displays user info"))
      .command("ping", |c| c
        .exec(commands::general::ping)
        .bucket("main")
        .desc("Check if the bot works"))
      .command("cleverbot", |c| c
        .exec(commands::general::cleverbot)
        .bucket("main")
        .known_as("talk")
        .desc("Have a nice converstion with Maika")))
    .group("Random", |g| g
      .command("choose", |c| c
        .exec(commands::random::choose)
        .bucket("main")
        .desc("Choose between options"))
      .command("coinflip", |c| c
        .exec(commands::random::coinflip)
        .bucket("main")
        .known_as("cf")
        .desc("Flip a coin"))
      .command("8ball", |c| c
        .exec(commands::random::magic_eight_ball)
        .bucket("main")
        .desc("8ball"))
      .command("roll", |c| c
        .exec(commands::random::roll)
        .bucket("main")
        .desc("Roll a random number"))
      .command("roulette", |c| c
        .exec(commands::random::roulette)
        .bucket("main")
        .desc("Roulette")))
    .group("Otaku", |g| g
      .command("anime", |c| c
        .exec(commands::otaku::anime)
        .bucket("main")
        .desc("Shows info about an anime")))
  );

  if let Err(err) = client.start() {
    utils::logger::error(format!("Client error:\n{:?}", err));
  }
}
