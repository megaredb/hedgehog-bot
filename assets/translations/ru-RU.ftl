start = 
    👋 <b>Привет!</b>

    Этот бот создан специально для 🦔 <a href="https://t.me/Mushoku_Tensei_AudioBook">HEDGEHOG.INC</a>.
    Для привязки своего аккаунта Boosty введите команду /email с вашей почтой.

    <i>Пример:</i> <code>/email test.mail@example.com</code>

    📱 <b>Контакты:</b>
    Канал автора контента: <i><a href="https://t.me/Mushoku_Tensei_AudioBook">Реинкарнация Безработного</a></i>
    Разработчик бота: <i><a href="https://t.me/megaredb">megared</a></i>
invalid-email =
    ❌ Почта указана неверно.
user-found = 
    ✅ <b>Мы нашли пользователя с почтой {$email}.</b>

    Имя: <i>{$name}</i>
    Уровень подписки: <i>{$level}</i>

    Теперь этот аккаунт привязан к вашему Telegram. Используйте команду /profile для просмотра данных.
    
    Ссылка для вступления в группу: https://t.me/+nj3Egg0X1ZxiN2Ji
user-not-subscribed = 
    ⚠️ <b>Мы нашли пользователя с почтой {$email}, но его аккаунт <b>не имеет</b> активной подписки.</b>
user-already-exists = 
    ⚠️ <b>Данный аккаунт уже существует и привязан к аккаунту Telegram.</b>

    Если считаете, что это ошибка, обратитесь к разработчику бота.
no-user-found = 
    ❌ <b>Мы не нашли пользователя с почтой {$email}.</b>

    Если это ошибка, свяжитесь с разработчиком бота.

no-profile =
    ❌ <b>Ваш Telegram аккаунт не привязан к аккаунту Boosty.</b>
    
    Введите команду /start или /help для помощи.
profile-api-error = 
    ❌ <b>Возникла ошибка при попытке найти Ваш профиль на Boosty.</b>

    Возможные причины:
    - Бот не смог обратиться к Boosty.
    - Вы отменили подписку на автора либо что-то произошло с Вашим аккаунтом Boosty.

    Если Вы не отменяли подписку и не производили махинаций с аккаунтом, обратитесь к разработчику бота.
profile = 
    👤 <b>Профиль</b>
    
    🏷 Имя: <i>{$name}</i>
    📬 Почта: <i>{$email}</i> 
    🤩 Уровень подписки: <i>{$level}</i>
    💸 Стоимость: <i>{$price} RUB</i>
    🗓 Действует до: <i>{$expires-at}</i>

    ⚠️ <i>Внимание! Значение поля "Действует до" является приблизительным значением, которое может отличаться от действительного.</i>

    Ссылка для вступления в группу: https://t.me/+nj3Egg0X1ZxiN2Ji