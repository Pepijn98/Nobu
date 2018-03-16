use colored::*;

pub fn command(guild: &String, user: &String, msg: &str) {
    println!("{} >> {} > {}", guild.green().bold(), user.magenta().bold(), msg.cyan().bold());
}

pub fn command_dm(guild: &str, user: &String, msg: &str) {
    println!("{} >> {} > {}", guild.green().bold(), user.magenta().bold(), msg.cyan().bold());
}

pub fn error(msg: String) {
    println!("{}", msg.red().bold());
}

pub fn info(msg: String) {
	println!("{}", msg.green().bold());
}

/* Not yet used
pub fn warn(msg: String) {
  println!("{}", msg.yellow().bold());
}
*/