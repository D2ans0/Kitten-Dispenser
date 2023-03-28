use serenity::model::prelude::Member;
use serenity::prelude::Mentionable;
use tracing::error;
use serenity::builder::CreateApplicationCommand;
use serenity::futures::StreamExt;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use rand::seq::SliceRandom;

pub async fn run(options: &[CommandDataOption], ctx: &Context, interaction: &ApplicationCommandInteraction, config: &crate::helpers::config_reader::KittenConfig) -> String {
    let role = options
        .get(0)
        .expect("Expected role option")
        .resolved
        .as_ref()
        .expect("Expected role object");

    if !config.bot_admins.contains_key(&interaction.user.id.to_string()) {
        return config.non_admin_response.clone();
    }

    match interaction.defer(&ctx).await {
        Ok(result) => result,
        Err(e) => error!("{:#?}", &e),
    }

    match role {
        CommandDataOptionValue::Role(role) => {
            let mut member_list: Vec<Member> = vec![];
            let guild = interaction.guild_id.unwrap();
            let members = guild.members_iter(&ctx);
            let mut members_boxed = guild.members_iter(&ctx).boxed();


            while let Some(member_result) = members_boxed.next().await {
                match member_result {
                    Ok(member) => {
                        if member.user.bot { continue; };
                        member_list.push(member.clone());
                    },
                    Err(error) => error!("Uh oh!  Error: {}", error),
                }
            }

            let _map = members.filter_map(|i| {
                let mut member = i.unwrap();
                async move {
                    if member.roles.contains(&role.id) {
                        member.remove_role(&ctx, &role.id).await.ok().unwrap();
                    }
                    Some(member)
                }
            }).collect::<Vec<_>>().await;

            let random_member = member_list.choose(&mut rand::thread_rng()).clone();
            match random_member.unwrap().clone().add_role(&ctx, &role.id).await {
                Ok(_result) => {
                    let message = format!("{} is now {}", random_member.unwrap().mention().to_string(), role.mention().to_string());
                    interaction.create_followup_message(&ctx, |f| {
                        f.content(&message)
                    }).await.unwrap();
                    return message
                },
                Err(error) => {
                    error!("it broky: {:#?}", error.to_string());
                    return "An error occured".to_string()
                }
            }
        }
        _ => return "Please provide a valid user".to_string(),
    }
}

pub fn register<'a>(command: &'a mut CreateApplicationCommand, name: &'a str) -> &'a mut CreateApplicationCommand {
    command
        .name(name)
        .description(name)
        .create_option(|option| {
            option
                .name("role")
                .description("Role to randomize")
                .kind(CommandOptionType::Role)
                .required(true)
        })
}