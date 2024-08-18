Remmy is (currently) a bot that can cross post text based stuff from Reddit to Lemmy.

It's WIP, soo, it doesn't have too many features yet. Feel free to use it and modify it for your own use cases, as it's under the Unlicense license.

Oh, usage!:

For now (it's not too clean yet, I'll make it use a config file in the future), define these environment variables in your system and run the program with "cargo r -r":

- REDDIT_CLIENT_ID -> The ID of your Reddit App (Look up what it is if you don't know what I mean)
- REDDIT_CLIENT_SECRET -> The secret of your Reddit App
- REDDIT_USERNAME -> Your Reddit account's username that this bot will use
- REDDIT_PASSWORD -> Your Reddit account's password
- SUBREDDIT -> The subreddit that you want to do stuff with
- LEMMY_INSTANCE -> The Lemmy instance that you want to connect to and use. Your bot should be on the same instance. Give this variable a value such as: Cryptolancing@Lemmy.wtf
- LEMMY_USERNAME -> The username of the bot
- LEMMY_PASSWORD -> The password of the bot
- LEMMY_COMMUNITY -> The Lemmy community that you want to do stuff with

After executing, the bot will take 5 top posts of the day of the subreddit of your choice, post it (the text for now) on the Lemmy community of your choice, and it'll do it on loop once on every day.
