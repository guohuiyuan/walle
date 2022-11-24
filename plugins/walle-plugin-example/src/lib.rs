use walle::{
    builtin::{strip_prefix,to_me,strip_whitespace,start_with},
    Rule,
    may_fail_handler_fn,
    walle_core::{
        event::{Message, MessageDeatilTypes},
    },
    Matcher, MatcherHandlerExt, PreHandler, ReplyAbleSession, Session,
};

// prefix_matcher 前缀匹配
pub fn prefix_matcher() -> Matcher {
    strip_prefix("hello")
        .layer(may_fail_handler_fn(
            |s: &Session<Message, MessageDeatilTypes>| {
                Box::pin(async move {
                    s.send("hello world!").await.ok();
                    Ok::<_, String>(())
                })
            },
        ))
        .boxed()
}

// on_to_me @bot的消息
pub fn on_to_me() -> Matcher {
    strip_whitespace().with(to_me()).layer(may_fail_handler_fn(
        |s: &Session<Message, MessageDeatilTypes>| {
            Box::pin(async move {
                s.send("这是一条@bot的消息").await.ok();
                Ok::<_, String>(())
            })
        },
    ))
    .boxed()
}


// prefix 你好
pub fn prefix() -> Matcher {
    start_with("你好").layer(may_fail_handler_fn(
        |s: &Session<Message, MessageDeatilTypes>| {
            Box::pin(async move {
                s.send("以你好开头").await.ok();
                Ok::<_, String>(())
            })
        },
    ))
    .boxed()
}

