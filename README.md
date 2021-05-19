# Trapperu Real

Trapperu Real (Romanian for "The Real Trapper") is a telegram bot made to annoy the members of your groups with stupid stuff.

Note: The responses of the bot are all in Romanian and they may contain vulgar language.

## What does the bot do?

* /joaco - Iane, joaco (Romanian for "[Ian](https://www.youtube.com/channel/UCXEsaxE4BOzgDKa2kG48koA), play it")
* /adauga [Group_alias]\~[Expression]\~[message] - Add an expression that everytime it evaluates to true, the bot responds with the given message. You may ommit the group alias
* /taci - Shut down the bot (but only if you're the admin)
* /help - Offers information about other commands
* /alias - Returns the group id and the group alias
* /alias [Alias] - Sets the group alias to the parameter
* /gindeste [Alias]~[Thought] - gindeste (gandeste, Romanian for "think!"), adds a thought to the memory of the bot
* /gind - gind (gand, Romanian for "thought") returns a memorised thought, display it on the chat and then deletes it from its memory

## Installation

Set the following environment variables in the following way:

> ADMIN_ID=[your Telegram ID here]

> TELOXIDE_TOKEN=[your Telegram token goes here]

> BOT_NAME=[your bot's Telegram handle]

And then just use:

> cargo run

If you want a one-liner command, you should do:

> ADMIN_ID=[your Telegram ID here] TELOXIDE_TOKEN=[your Telegram token goes here] BOT_NAME=[your bot's Telegram handle] cargo run
