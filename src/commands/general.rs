use serenity::utils::Colour;
use chrono;
use time::PreciseTime;
use cleverbot_io::Cleverbot;
use utils::logger;
use std::env;
use serenity::client::{Context};
use serenity::framework::standard::{Args, CommandError};
use serenity::model::Message;
/*
command!(ping(_ctx, msg, _args) {
  let start = PreciseTime::now();
  let mut msg = match msg.channel_id.say("Pong!") {
    Ok(msg) => msg,
    Err(_) => return Ok(()),
  };
  let end = PreciseTime::now();
  let ms = start.to(end).num_milliseconds();
  let _ = msg.edit(|m| m.content(&format!("Pong!, {} milliseconds", ms)));
});
*/

pub fn ping(_ctx: &mut Context, msg: &Message, _args: Args) -> Result<(), CommandError> {
    let start = PreciseTime::now();
    let mut msg = match msg.channel_id.say("Pong!") {
        Ok(msg) => msg,
        Err(_) => return Ok(()),
    };
    let end = PreciseTime::now();
    let ms = start.to(end).num_milliseconds();
    let _ = msg.edit(|m| m.content(&format!("Pong!, {} milliseconds", ms)));

    Ok(())
}

pub fn userinfo(_ctx: &mut Context, msg: &Message, _args: Args) -> Result<(), CommandError> {
  if let Some(guild) = msg.guild() {
    let guild = guild.read();
      
    if let Some(member) = guild.members.get(&msg.author.id) {
      let mut roles = "@every\u{200b}one".to_owned();
      let mut iter = member.roles.iter();
      while let Some(role_id) = iter.next() {
        if let Some(role) = role_id.find() {
          roles.push_str(", ");
          roles.push_str(&role.name);
        } else {
          return Err(CommandError::from("No RoleId for this Role".to_string()));
        }
      }

      let joined_at = {
        if let Some(join_date) = member.joined_at.as_ref() {
          join_date.naive_utc().format("%c")
        } else {
          chrono::naive::NaiveDateTime::from_timestamp(0, 0).format("%c") 
        }
      };
      let avatar_url = msg.author.face();
      let id = msg.author.id.0.to_string();
      let nick = member.nick.as_ref().unwrap_or_else(|| &msg.author.name);
      let dtag = msg.author.tag();
      let created_at = msg.author.created_at().format("%c").to_string();
      let footer_text = format!("Member since {}", joined_at);

      let _ = match msg.channel_id.send_message(|cm| cm.embed(|ce|
        ce.author(|cea| cea.name(&dtag).icon_url(&avatar_url))
          .title("Info")
          .field("ID", &id, true)
          .field("Current Name", nick, true)
          .field("Created at", &created_at, true)
          .field("Roles", &roles, true)
          .footer(|cef| cef.text(&footer_text))
          .image(&avatar_url)
          .color(Colour::from_rgb(246, 219, 216))
      )){
        Ok(msg) => msg,
        Err(why) => {
          logger::error(format!("{:?}", why));
          let _ = msg.channel_id.say(format!("Error sending embed:\n{:?}", why));
          return Ok(());
        },
      };
    }
  }

  Ok(())
}

pub fn cleverbot(_ctx: &mut Context, msg: &Message, args: Args) -> Result<(), CommandError> {
  if args.is_empty() {
    let _ = msg.channel_id.say("What do you want to talk about?");
    return Ok(());
  }

  let query = args.join(" ");

  let _ = msg.channel_id.broadcast_typing();

  let mut bot = Cleverbot::new(env::var("CLEVERBOT_USER").unwrap(), env::var("CLEVERBOT_KEY").unwrap(), None).unwrap();
  let res = bot.say(&query).unwrap();
  let m = msg.channel_id.say(res);
  if m.is_err() {
    logger::error(format!("Error sending message\n{:?}", m));
    return Ok(());
  }

  Ok(())
}