Remmy is (currently) a bot that can cross post text based stuff from Reddit to Lemmy.

It's WIP, soo, it doesn't have too many features yet. Feel free to use it and modify it for your own use cases, as it's under the Unlicense license.

Oh, usage!:

Create a config file in yaml, such as remmy.yaml, and then execute the program like so:
```sh
cargo r -r -- -c remmy.yaml
```

Your remmy.yaml file should look something like this:
```yaml
reddit:
  client_id: REDDIT_CLIENT_ID
  client_secret: REDDIT_CLIENT_SECRET
  username: REDDIT_USERNAME
  password: REDDIT_PASSWORD
  subreddit: SUBREDDIT

lemmy:
  instance: LEMMY_INSTANCE
  community: LEMMY_COMMUNITY
  username: LEMMY_USERNAME
  password: LEMMY_PASSWORD
```

- REDDIT_CLIENT_ID -> The ID of your Reddit App (Look up what it is if you don't know what I mean)
- REDDIT_CLIENT_SECRET -> The secret of your Reddit App
- REDDIT_USERNAME -> Your Reddit account's username that this bot will use
- REDDIT_PASSWORD -> Your Reddit account's password
- SUBREDDIT -> The subreddit that you want to do stuff with
- LEMMY_INSTANCE -> The Lemmy instance that you want to connect to and use. Your bot should be on the same instance.
- LEMMY_COMMUNITY -> The Lemmy community that you want to do stuff with. Give this a value such as: Cryptolancing@Lemmy.wtf
- LEMMY_USERNAME -> The username of the bot
- LEMMY_PASSWORD -> The password of the bot

Command line options:

```sh
Command-Line interface for Remmy A Reddit to Lemmy bot

Usage: remmy [OPTIONS] --config <CONFIG>

Options:
  -c, --config <CONFIG>          Path to config file
  -n, --num <NUM>                Number of posts to make [default: 5]
  -d, --dry-run                  Whether to actually post anything or not Useful for seeing what
                                 posts WILL be made if you were torun this program
  -w, --wait-time <WAIT_TIME>    How long should I wait before posting again? (in seconds)
                                 Default is 86400 seconds or 1 day [default: 86400]
  -r, --retry-time <RETRY_TIME>  How long should I wait before retrying to post after an error?
                                 (in seconds) [default: 5]
  -s, --sorting <SORTING>        How to sort the posts of the source platform? [default: top]
                                 [possible values: top, hot, new, controversial]
  -t, --time-frame <TIME_FRAME>  what time frame to use for the posts of the source platform?
                                 [default: day] [possible values: hour, day, week, month, year,
                                 all]
  -h, --help                     Print help
```

After executing, (by default and if no other cli option is specified) the bot will take 5 top posts of the day of the subreddit of your choice, post it (the text for now) on the Lemmy community of your choice, and it'll do it on loop once every day.
