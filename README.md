[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fhouse-of-vanity%2Fdesubot.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fhouse-of-vanity%2Fdesubot?ref=badge_shield)

Desubot
Telegram bot with light group statistic and heavy spy features.

== Features ==
* Collect all the messages sent to group.
* Collect all the media sent to group including voice, stickers, video, video notes, documents.
* /here command to mention all members.
* Alongside with saving whole message bot perform blacklist filter and stemming for every word (only Russian). "Красивую собаку мыли негры" -> "красивый собака мыть негр"
* Generate sentences using Markov Chains trained on history with /markov_all.
* Syntax highlighting for CODE exported to image.

== Important ==
* Desubot uses MyStem by Yandex for word stemming and assume that mystem binary is available in PATH.
* ubuntu deps: libssl-dev libsqlite3-dev cmake libfreetype-dev


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fhouse-of-vanity%2Fdesubot.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fhouse-of-vanity%2Fdesubot?ref=badge_large)
