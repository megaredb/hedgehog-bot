use teloxide::{
    requests::{Requester, ResponseResult},
    types::{ChatId, ChatJoinRequest, UserId},
};

use crate::{
    boosty_api::{
        types::subscribers::{Order, SortBy, SubscribersRequest},
        BoostyClient,
    },
    db::{
        users::{get_user, get_user_by_boosty_id, get_users_boosty_ids, remove_user},
        Pool,
    },
    utils::{Bot, Config},
};

pub async fn chat_join_handler(
    boosty_client: BoostyClient,
    pool: Pool,
    config: Config,
    bot: Bot,
    chat_join_request: ChatJoinRequest,
) -> ResponseResult<()> {
    let chat_id = chat_join_request.chat.id;

    if chat_id.0 != config.chat_id {
        return Ok(());
    }

    let from_user_id = chat_join_request.from.id;

    let mut conn = pool.get().await.unwrap();

    let decline = || async { bot.decline_chat_join_request(chat_id, from_user_id).await };

    let user_result = get_user(&mut conn, from_user_id.0 as i64).await;

    if user_result.is_err() {
        decline().await?;

        return Ok(());
    }

    let user = user_result.unwrap();

    let res = boosty_client
        .subscribers(&SubscribersRequest {
            user_ids: vec![user.boosty_id as u64].into(),
            sort_by: SortBy::default(),
            limit: 11,
            offset: None,
            order: Order::default(),
        })
        .await;

    if res.is_err() {
        decline().await?;

        return Ok(());
    }

    let boosty_users = res.unwrap().data;

    if boosty_users.is_empty() {
        decline().await?;

        return Ok(());
    }

    if !boosty_users.last().unwrap().is_paid() {
        decline().await?;

        return Ok(());
    }

    bot.approve_chat_join_request(chat_id, from_user_id).await?;

    Ok(())
}

pub async fn chat_subscribers_checker(
    boosty_client: BoostyClient,
    pool: Pool,
    config: Config,
    bot: Bot,
) -> ResponseResult<()> {
    let mut conn = pool.get().await.unwrap();

    let boosty_ids: Vec<u64> = get_users_boosty_ids(&mut conn)
        .await
        .unwrap()
        .iter()
        .map(|value| *value as _)
        .collect();

    let boosty_users = boosty_client
        .subscribers(&SubscribersRequest {
            user_ids: boosty_ids.into(),
            sort_by: SortBy::default(),
            limit: 100,
            offset: None,
            order: Order::default(),
        })
        .await
        .unwrap()
        .data;

    for boosty_user in boosty_users {
        if boosty_user.is_paid() {
            continue;
        }

        let db_result = get_user_by_boosty_id(&mut conn, boosty_user.basic_info.id as i64).await;

        let chat_id = ChatId(config.chat_id);

        if let Ok(user) = db_result {
            remove_user(&mut conn, user.id).await.unwrap();

            let user_id = UserId(user.id as u64);

            let chat_member = bot.get_chat_member(chat_id, user_id).await;

            if chat_member.is_err() {
                continue;
            }

            let chat_member = chat_member.unwrap();

            if chat_member.is_administrator() || !chat_member.is_present() {
                continue;
            }

            bot.kick_chat_member(chat_id, user_id).await.unwrap();
        }
    }

    Ok(())
}
