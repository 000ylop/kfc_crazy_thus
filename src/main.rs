use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
};

use chrono::{Datelike, FixedOffset, TimeZone, Weekday};
use daily_material::{talent, weapon};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env().auto_send();
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |query: InlineQuery, bot: AutoSend<Bot>| async move {
            let time = get_weekday().await;

            let talent = get_talent(time).await;
            let weapon = get_weapon(time).await;
            let talent_next = get_talent(time.succ()).await;
            let weapon_next = get_weapon(time.succ()).await;

            let content_text = InputMessageContentText::new(format!(
                "{talent}\n{weapon}\n\n次日\n{talent_next}\n{weapon_next}"
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
            let result = vec![talent_text, weapon_text];

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

async fn get_talent(time: Weekday) -> String {
    let list = match time {
        Weekday::Mon | Weekday::Thu => talent::MON_THU,
        Weekday::Tue | Weekday::Fri => talent::TUE_FRI,
        Weekday::Wed | Weekday::Sat => talent::WED_SAT,
        _ => return "周日随便刷哦".to_owned(),
    };
    format!("天赋材料：{}", list.join(" "))
}

async fn get_weapon(time: Weekday) -> String {
    let list = match time {
        Weekday::Mon | Weekday::Thu => weapon::MON_THU,
        Weekday::Tue | Weekday::Fri => weapon::TUE_FRI,
        Weekday::Wed | Weekday::Sat => weapon::WED_SAT,
        _ => return "周日随便刷哦".to_owned(),
    };
    format!("武器材料：{}", list.join(" "))
}

async fn get_weekday() -> Weekday {
    let utc = &chrono::Utc::now().naive_utc();
    let datetime = FixedOffset::east(4 * 3600).from_utc_datetime(utc);
    datetime.weekday()
}
