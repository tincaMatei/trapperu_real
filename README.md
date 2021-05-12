# Trapperu Real

Trapperu Real (Romanian for "The Real Trapper") is a telegram bot made to annoy the members of your groups with stupid stuff.

## What does the bot do?

* /joaco - Iane, joaco (Romanian for "[Ian](https://www.youtube.com/channel/UCXEsaxE4BOzgDKa2kG48koA), play it")
* /adauga [Expression]~[message] - Add an expression that everytime it evaluates to true, the bot responds with the given message.
* /taci - Shut down the bot (but only if you're the admin)

## Installation

Set the following environment variables in the following way:

> ADMIN_ID=[your Telegram ID here]

> TELOXIDE_TOKEN=[your Telegram token goes here]

> BOT_NAME=[your bot's Telegram handle]

And then just use:

> cargo run

If you want a one-liner command, you should do:

> ADMIN_ID=[your Telegram ID here] TELOXIDE_TOKEN=[your Telegram token goes here] BOT_NAME=[your bot's Telegram handle] cargo run
