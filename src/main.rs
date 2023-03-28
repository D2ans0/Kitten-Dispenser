mod commands;
mod helpers;
use tracing::{debug, error, warn, info};
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler {
    configuration: helpers::config_reader::KittenConfig,
}

const RANDOMIZE_ROLE: &str = "randomize_role";

#[async_trait]
impl EventHandler for Handler {
    
    
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            debug!("Received command interaction: {:#?}", command);
            let content = match command.data.name.as_str() {
                "id" => commands::id::run(&command.data.options),
                RANDOMIZE_ROLE => commands::randomize_role::run(&command.data.options, &ctx, &command, &self.configuration).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                warn!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::randomize_role::register(command, RANDOMIZE_ROLE))
                .create_application_command(|command| commands::id::register(command))
        }).await;


        debug !("Added slash commands: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() {
    let args = helpers::arguments::get();
    let config = helpers::config_reader::get(args.value_of("config").unwrap_or("config.yaml").into());
    let token: &String = &config.token;

    // helpers::tracing_helper::start(config.get_string("logging_level").unwrap_or("EMPTY".to_string()));
    helpers::tracing_helper::start(&config.verbosity);


    let handler = Handler {
        // admins: config.bot_admins
        configuration: config.clone(),
    };

    // config_reader::get(path);

    // Configure the client with your Discord bot token in the environment.
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
