use jsj_backend::{has_sound};
use jsj_backend::database::{create_new_joinsound,update_joinsound};
use argparse::{ArgumentParser, StoreOption};

fn main() {
    dotenv::dotenv().ok();
    let mut user_id: Option<String> = None;
    let mut guild_id: Option<String> = None;
    let mut file_path: Option<String> = None;
    let mut url: Option<String> = None;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Set a join sound.");
        ap.refer(&mut user_id)
            .add_option(&["--user-id"], StoreOption,
            "Discord ID of the user");
        ap.refer(&mut guild_id)
            .add_option(&["--guild-id"], StoreOption,
            "Discord ID of the server [optional]");
        ap.refer(&mut file_path)
            .add_option(&["--file-path"], StoreOption,
            "Path of the file [THIS ASSUMES THE FILE IS ALREADY THERE]");
        ap.refer(&mut url)
            .add_option(&["--url"], StoreOption,
            "Youtube url");
        ap.parse_args_or_exit();
    }

    let parsed_user_id = poise::serenity_prelude::UserId(user_id.unwrap().parse::<u64>().ok().unwrap());
    let parsed_guild_id = match guild_id {
        Some(g) => Some(poise::serenity_prelude::GuildId(g.parse::<u64>().ok().unwrap())),
        None => None
    };
    let parsed_file_path = file_path.unwrap();
    let parsed_url = url.unwrap();

    if has_sound(parsed_user_id, parsed_guild_id) {
        update_joinsound(parsed_user_id, parsed_guild_id, parsed_file_path, parsed_url);
    }
    else 
    {
        create_new_joinsound(parsed_user_id, parsed_guild_id, parsed_file_path, parsed_url)    
    }
}
