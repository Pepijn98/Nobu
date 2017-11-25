use serenity::client::Context;
use serenity::framework::standard::{Args, CommandError};
use serenity::model::Message;
use rand::{self, Rng};

pub fn choose(_ctx: &mut Context, msg: &Message, args: Args) -> Result<(), CommandError> {
  let query = args.join(" ");
  let mut choices: Vec<&str> = query.split(", ").collect::<Vec<&str>>();

  if choices.len() < 2 {
    choices = query.split(' ').collect();
  }

  choices.sort();
  choices.dedup();

  if choices.len() < 2 {
    let _ = msg.channel_id.say("Must have at least 2 choices");

    return Ok(());
  }

  let _ = match rand::thread_rng().choose(&choices) {
    Some(choice) => msg.channel_id.say(&choice[..]),
    None => msg.channel_id.say("No choice found"),
  };

  Ok(())
}

pub fn coinflip(_ctx: &mut Context, msg: &Message, _args: Args) -> Result<(), CommandError> {
  let num = rand::thread_rng().gen::<u8>();

  let _ = msg.channel_id.say(match num {
    0 ... 126 => "Heads",
    128 ... 255 => "Tails",
    _ => "On its side",
  });

  Ok(())
}

pub fn magic_eight_ball(_ctx: &mut Context, msg: &Message, args: Args) -> Result<(), CommandError> {
  if args.is_empty() {
    let _ = msg.channel_id.say("Please specify some arguments");

    return Ok(());
  }
    let answers: [&'static str; 14] = [
      "It is certain",
      "Most likely",
      "Outlook good",
      "Without a doubt",
      "Yes",
      "You may rely on it",
      "Better not tell you now",
      "Reply hazy, try again",
      "Absolutely not",
      "Don't count on it",
      "My reply is no",
      "My sources say no",
      "Outlook not so good",
      "Very doubtful",
    ];

    let _ = match rand::thread_rng().choose(&answers) {
      Some(answer) => msg.channel_id.say(answer),
      None => msg.channel_id.say("No answer found"),
    };

  Ok(())
}

pub fn roll(_ctx: &mut Context, msg: &Message, args: Args) -> Result<(), CommandError> {
  if !args.is_empty() && args.len() != 2 {
    let _ = msg.channel_id.say("Either 0 or 2 numbers must be given");

    return Ok(());
  }

  let nums = {
    if args.is_empty() {
      [1, 6]
    } else {
      let (arg1, arg2) = unsafe {
        (args.get_unchecked(0), args.get_unchecked(1))
      };

     let arg1 = match arg1.parse::<isize>() {
      Ok(arg1) => arg1,
      Err(_) => {
        let _ = msg.channel_id.say(&format!("{} is not an integer", arg1));

        return Ok(());
      },
    };
    let arg2 = match arg2.parse::<isize>() {
      Ok(arg2) => arg2,
      Err(_) => {
        let _ = msg.channel_id.say(&format!("{} is not an integer", arg2));

        return Ok(());
      },
    };

    let mut nums = vec![arg1, arg2];
      nums.sort();

      [nums[0], nums[1]]
    }
  };

  if nums[0] == nums[1] {
    let _ = msg.channel_id.say("The given integers can not be equal");

    return Ok(());
  }

  let number = rand::thread_rng().gen_range(nums[0], nums[1]);

  let _ = msg.channel_id.say(&number.to_string());

  Ok(())
}

pub fn roulette(_ctx: &mut Context, msg: &Message, _args: Args) -> Result<(), CommandError> {
  let result = if rand::thread_rng().gen_range(0, 6) == 0 {
    format!("BANG! {} was shot", msg.author)
  } else {
    r"\*click\*".to_owned()
  };

  let _ = msg.channel_id.say(&result);

  Ok(())
}
