use log::info;
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult::{self, Article},
        InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
};

use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike, Weekday};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env().auto_send();
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |query: InlineQuery, bot: AutoSend<Bot>| async move {
            let result = get_materials();
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

    Dispatcher::builder(bot, handler).build().dispatch().await;

    info!("Bot started successfully");
}

fn utc_plus_8() -> DateTime<FixedOffset> {
    let utc = &chrono::Utc::now().naive_utc();
    FixedOffset::east(8 * 3600).from_utc_datetime(utc)
}

fn left_time_from_thus(time: DateTime<FixedOffset>) -> InlineQueryResult {
    let (rsec, rmin, rhour) = match (time.second(), time.minute(), time.hour()) {
        (0, 0, hour) => (0, 0, 24 - hour),
        (0, min, hour) => (0, 60 - min, 23 - hour),
        (sec, min, hour) => (60 - sec, 59 - min, 23 - hour),
    };

    let info = if time.weekday() == Weekday::Thu {
        format!("疯狂星期四耶！\n还有 {rhour:02}:{rmin:02}:{rsec:02}就结束啦~")
    } else {
        let diff_day = match time.weekday() {
            Weekday::Mon => "三",
            Weekday::Tue => "两",
            Weekday::Wed => "一",
            Weekday::Fri => "六",
            Weekday::Sat => "五",
            Weekday::Sun => "四",

            Weekday::Thu => unreachable!(),
        };
        format!("失落……\n还要{diff_day}天零{rhour:02}:{rmin:02}:{rsec:02}才能开始 呜呜呜")
    };

    let content = InputMessageContent::Text(InputMessageContentText::new(&info));
    Article(InlineQueryResultArticle::new("rest", info, content))
}

#[inline(always)]
fn get_materials() -> Vec<InlineQueryResult> {
    let datetime = utc_plus_8();
    vec![left_time_from_thus(datetime)]
}
