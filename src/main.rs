use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
};

use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike, Weekday};
use daily_material::{talent, weapon};

const SUN_TIP: &str = "周日随便刷哦";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env().auto_send();
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |query: InlineQuery, bot: AutoSend<Bot>| async move {
            let datetime = get_datetime();
            let weekday = datetime.weekday();
            let (rsec, rmin, rhour) = get_remaining(datetime);

            let talent = get_talent(weekday);
            let weapon = get_weapon(weekday);
            let talent_next = get_talent(weekday.succ());
            let weapon_next = get_weapon(weekday.succ());

            let content_text = InputMessageContentText::new(format!(
                "{talent}\n{weapon}\n\n{rhour:02}:{rmin:02}:{rsec:02}后：\n{talent_next}\n{weapon_next}"
            ));
            let content = InputMessageContent::Text(content_text);

            let talent_text = InlineQueryResult::Article(InlineQueryResultArticle::new(
                "天赋材料",
                talent,
                content.clone(),
            ));
            let weapon_text = InlineQueryResult::Article(InlineQueryResultArticle::new(
                "武器材料",
                weapon,
                content,
            ));
            let result = [talent_text, weapon_text];

            let response = bot
                .answer_inline_query(&query.id, result)
                .cache_time(0)
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
}

fn get_talent(time: Weekday) -> String {
    let list = match time {
        Weekday::Mon | Weekday::Thu => talent::MON_THU,
        Weekday::Tue | Weekday::Fri => talent::TUE_FRI,
        Weekday::Wed | Weekday::Sat => talent::WED_SAT,
        _ => return SUN_TIP.to_owned(),
    };
    format!("天赋材料：{}", list.join(" "))
}

fn get_weapon(time: Weekday) -> String {
    let list = match time {
        Weekday::Mon | Weekday::Thu => weapon::MON_THU,
        Weekday::Tue | Weekday::Fri => weapon::TUE_FRI,
        Weekday::Wed | Weekday::Sat => weapon::WED_SAT,
        _ => return SUN_TIP.to_owned(),
    };
    format!("武器材料：{}", list.join(" "))
}

fn get_datetime() -> DateTime<FixedOffset> {
    let utc = &chrono::Utc::now().naive_utc();
    FixedOffset::east(4 * 3600).from_utc_datetime(utc)
}

fn get_remaining(time: DateTime<FixedOffset>) -> (u32, u32, u32) {
    match (time.second(), time.minute(), time.hour()) {
        (0, 0, hour) => (0, 0, 24 - hour),
        (0, min, hour) => (0, 60 - min, 23 - hour),
        (sec, min, hour) => (60 - sec, 59 - min, 23 - hour),
    }
}
