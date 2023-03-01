use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    println!("aaaaaaaa");
    "It works!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("作業しろ")
        .description("VCに参加して、作業を促します。")
        .dm_permission(false)
}
