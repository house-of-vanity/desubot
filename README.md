[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fhouse-of-vanity%2Fdesubot.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fhouse-of-vanity%2Fdesubot?ref=badge_shield)

# Desubot Telegram Bot

**Desubot** is a Telegram bot with light group statistics and powerful spy features.

## Features

- **Collect all messages**: The bot collects all messages sent to the group.
- **Collect all media**: The bot saves all media sent to the group, including voice messages, stickers, videos, video notes, and documents.
- **/here command**: Mention all group members.
- **Blacklist filter and stemming**: The bot saves the entire message, performs blacklist filtering, and stems every word (Russian only). For example, "Красивую собаку мыли негры" -> "красивый собака мыть негр".
- **Markov Chain sentence generation**: The bot generates sentences using Markov Chains trained on the history with the `/markov_all` command.
- **Syntax highlighting for CODE**: Export code with syntax highlighting to an image.

## Important

- **MyStem**: Desubot uses MyStem by Yandex for word stemming and assumes that the `mystem` binary is available in the PATH.
- **Ubuntu dependencies**: The following packages are required:

  ```bash
  libssl-dev libsqlite3-dev cmake libfreetype-dev pkg-config


[Docker Hub](https://hub.docker.com/repository/docker/ultradesu/desubot/general)


![image](https://user-images.githubusercontent.com/4666566/150677613-32bdedf9-4b4c-4ec5-99cd-3d0221e56fb5.png)

![image](https://user-images.githubusercontent.com/4666566/150677660-183572b4-2a69-425f-a32c-dba5ec97e438.png)

## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fhouse-of-vanity%2Fdesubot.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fhouse-of-vanity%2Fdesubot?ref=badge_large)

