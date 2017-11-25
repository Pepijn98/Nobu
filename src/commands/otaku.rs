use serenity::client::Context;
use serenity::framework::standard::{Args, CommandError};
use serenity::model::Message;
use serenity::utils::Colour;
use kitsu_io;
use utils::logger;

pub fn anime(_ctx: &mut Context, msg: &Message, args: Args) -> Result<(), CommandError> {
  if args.is_empty() {
    let _ = msg.channel_id.say("Which anime should I search for?");
    return Ok(());
  }

  let search = args.join(" ");

  if let Ok(result) = kitsu_io::search_anime(|f| f.filter("text", &search)) {
    if let Some(anime) = result.data.get(0) {
      let anime_title = &anime.attributes.canonical_title;
      let anime_synopsis = &anime.attributes.synopsis;
      let anime_age_rating = match anime.attributes.age_rating {
        Some(ref x) => format!("{:?}", x),
        None => "-".to_owned(),
      };
      let anime_average_rating = match anime.attributes.average_rating {
        Some(x) => (((x * 100_f64).round())/100_f64).to_string(),
        None => "-".to_owned(),
      };
      let anime_type = match anime.attributes.kind.name(){
        Ok(x) => x,
        Err(_) => "-".to_owned(),
      };
      let anime_airing_status = anime.attributes.airing_status();
      let anime_airing_status_name = anime_airing_status.name();
      let anime_episode_count = match anime.attributes.episode_count {
        Some(x) => x.to_string(),
        None => "-".to_owned(),
      };
      let anime_start_date = &anime.attributes.start_date;
      let anime_end_date = match anime.attributes.end_date {
        Some(ref x) => x.to_owned(),
        None => "?".to_owned(),
      };

      let anime_poster_image = match anime.attributes.poster_image.largest(){
        Some(x) => x,
        None => "",
      };

      let _ = match msg.channel_id.send_message(|cm| cm.embed(|ce| 
        ce.title(&anime_title)
          .url(&anime.url())
          .color(Colour::from_rgb(246, 219, 216))
          .description(&anime_synopsis)
          .thumbnail(anime_poster_image)
          .field("Average Rating", &anime_average_rating, true)
          .field("Type", &anime_type, true)
          .field("Age Rating", &anime_age_rating, true)
          .field("Episodes", &anime_episode_count, true)
          .field("Status", anime_airing_status_name, true)
          .field("Start/End", &format!("{} until {}", anime_start_date, &anime_end_date), true)
      )){
        Ok(msg) => msg,
        Err(why) => {
          logger::error(format!("{:?}", why));
          let _ = msg.channel_id.say(format!("Error sending embed:\n{:?}", why));
          return Ok(());
        },
      };
    } else {
      let _ = msg.channel_id.say("Failed to get anime info.");
    }
  } else {
    let _ = msg.channel_id.say("Failed to get anime info.");
  }

  Ok(())
}

pub fn manga(_ctx: &mut Context, msg: &Message, args: Args) -> Result<(), CommandError> {
  if args.is_empty() {
    let _ = msg.channel_id.say("Which manga should I search for?");
    return Ok(());
  }

  let search = args.join(" ");

  if let Ok(result) = kitsu_io::search_manga(|f| f.filter("text", &search)){
    if let Some(manga) = result.data.get(0) {
      let manga_title = &manga.attributes.canonical_title;
      let mut manga_synopsis = manga.attributes.synopsis.to_owned();
      if &manga_synopsis == "" {
        manga_synopsis = "-".to_owned();
      }
      let manga_type = &manga.attributes.kind;
      let manga_average_rating = match manga.attributes.average_rating {
        Some(x) => (((x * 100_f64).round())/100_f64).to_string(),
        None => "-".to_owned(),
      };
      let manga_volume_count = match manga.attributes.volume_count {
        Some(x) => x.to_string(),
        None => "-".to_owned(),
      };
      let manga_chapter_count = match manga.attributes.chapter_count{
        Some(x) => x.to_string(),
        None => "-".to_owned(),
      };
      let manga_start_date = match manga.attributes.start_date {
        Some(ref x) => x.to_owned(),
        None => "?".to_owned(),
      };
      let manga_end_date = match manga.attributes.end_date {
        Some(ref x) => x.to_owned(),
        None => "?".to_owned(),
      };
      let manga_poster_image = match manga.attributes.poster_image.largest(){
        Some(x) => x,
        None => "",
      };

      let _ = match msg.channel_id.send_message(|cm| cm.embed(|ce| 
        ce.title(&manga_title)
          .url(&manga.url())
          .colour(Colour::from_rgb(246, 219, 216))
          .description(&manga_synopsis)
          .thumbnail(manga_poster_image)
          .field("Average Rating", &manga_average_rating, true)
          .field("Type", &format!("{:?}", manga_type), true)
          .field("Volumes", &manga_volume_count, true)
          .field("Chapters", &manga_chapter_count, true)
          .field("Start/End", &format!("{} until {}", &manga_start_date, &manga_end_date), true)
        )){
        Ok(msg) => msg,
        Err(why) => {
          logger::error(format!("{:?}", why));
          let _ = msg.channel_id.say(format!("Error sending embed:\n{:?}", why));
          return Ok(());
        },
      };
    } else {
      let _ = msg.channel_id.say("Failed to get manga info.");
    }
  } else {
    let _ = msg.channel_id.say("Failed to get manga info.");
  }

  Ok(())
}