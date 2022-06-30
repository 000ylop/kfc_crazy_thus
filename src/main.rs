use log::info;
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult::{self, Article}, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
};

use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike, Weekday};
use daily_material::{talent, weapon, SUN_TIP, SPLITTER};


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env().auto_send();
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |query: InlineQuery, bot: AutoSend<Bot>| async move {
            let result = get_materials();
            let response = bot
                .answer_inline_query(&query.id, result)
                .cache_time(1)
                .send()
                .await;

            if let Err(e) = response {
                log::error!("Error in handler: {:?}", e);
            }

            respond(())
        },
    ));

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;

    info!("Bot started successfully");
}

fn talent(time: Weekday) -> String {
    let list = match time {
        Weekday::Mon | Weekday::Thu => talent::MON_THU,
        Weekday::Tue | Weekday::Fri => talent::TUE_FRI,
        Weekday::Wed | Weekday::Sat => talent::WED_SAT,
        _ => return SUN_TIP.to_owned(),
    };
    format!("天赋: {}", list.join(SPLITTER))
}

fn weapon(time: Weekday) -> String {
    let list = match time {
        Weekday::Mon | Weekday::Thu => weapon::MON_THU,
        Weekday::Tue | Weekday::Fri => weapon::TUE_FRI,
        Weekday::Wed | Weekday::Sat => weapon::WED_SAT,
        _ => return SUN_TIP.to_owned(),
    };
    format!("武器: {}", list.join(SPLITTER))
}

fn utc_plus_4() -> DateTime<FixedOffset> {
    let utc = &chrono::Utc::now().naive_utc();
    FixedOffset::east(4 * 3600).from_utc_datetime(utc)
}

fn rest_of_a_day(time: DateTime<FixedOffset>) -> (u32, u32, u32) {
    match (time.second(), time.minute(), time.hour()) {
        (0, 0, hour) => (0, 0, 24 - hour),
        (0, min, hour) => (0, 60 - min, 23 - hour),
        (sec, min, hour) => (60 - sec, 59 - min, 23 - hour),
    }
}

fn get_materials() -> [InlineQueryResult; 2] {
    let datetime = utc_plus_4();
    let weekday = datetime.weekday();
    let (rsec, rmin, rhour) = rest_of_a_day(datetime);

    let talent_list = talent(weekday);
    let weapon_list = weapon(weekday);
    let talent_next = talent(weekday.succ());
    let weapon_next = weapon(weekday.succ());

    let content_text = InputMessageContentText::new(format!(
        "{talent_list}\n{weapon_list}\n\n{rhour:02}:{rmin:02}:{rsec:02} 后:\n{talent_next}\n{weapon_next}"
    ));
    let content = InputMessageContent::Text(content_text);

    let talent_text = Article(InlineQueryResultArticle::new("天赋",talent_list,content.clone(),));
    let weapon_text = Article(InlineQueryResultArticle::new("武器", weapon_list, content));

    [talent_text, weapon_text]
}
