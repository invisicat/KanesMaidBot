# Kane's Maid

A funky discord bot made to do stuff. It's cool and everything.

[![Invite Kane][invite-badge]][invite-link]
[![License][license-badge]][license-link]
[![Dependency Status][dependency-badge]][dependency-link]
[![GitHub Actions Build Status][github-actions-badge]][github-actions-link]


## Features
- [ ] 📈 InfluxDB (Monitor statistics)
- [ ] ⏱ RedisDB (Interact from the outside world)
- [ ] 👮‍♂️ Moderation Commands
- [ ] 🎮 Game Commands
- [ ] 🔧 Utility Commands
- [ ] 🎹 Custom Commands
- [ ] 📊 Web dashboard
- [ ] 💵 Leveling System
- [ ] 🌠 Customized Rewards
- [ ] 🔕 Anti Spam
- [ ] ✅ Server Verification
- [ ] 📸 Image Generation
- [ ] 🎶 Music
- [ ] 📚 API Access (Reddit, Instagram, E.A, Steam, etc)
- [ ] 🎲 Game Integration (Apex Legends, Minecraft, Among Us, etc)
- [ ] 🈁 Localization (Translatable)

## Development

Kane's Maid is composed of many technologies and open source projects:

### Infrastructure
- Github for code management and version control
- Github actions for CI and CD
- Discord for discussions
- InfluxDB and Grafana for statistics
- RedisDB and RabbitMQ for scalable integrations between microservices
- My RaspberryPI hosting all this 🥧

### Runtimes
- RustLang for running the bot
- cargo for testing, running the dev service, building binaries and as the runtime harness
- Node.js for the web dashboard


### Semver
Kane's Maid is using [Semantic Versioning 2.0][semantic-versioning-link].

### Contributing

Contributions are welcome! Feel free to make a pull request to add any features that are appropriate. Though, make sure
you read up on the [Contributing Guide][github-contribution-link] before making one.

Thank you to all the people who already contributed to Kane's Maid!

### Licensing

Kane's Maid is licensed under the terms of the GPL-3.0, a fairly unrestrictive license that gives you the power to do
mostly anything you want with this project, and is one of two licenses used by the Rust project itself alongside version
2.0 of the Apache License, meaning that this software should be 100% compatible. The full contents of the GPL-3.0 license are
written in the `LICENSE` file, in the root project directory. Please read it in full to understand your full rights
with regards to this software.


<!-- Misc -->
[invite-link]: https://discordapp.com/oauth2/authorize?client_id=295007212638830592&scope=bot
[invite-badge]: https://img.shields.io/badge/Invite-to%20your%20Discord%20server-7289da.svg?style=flat-square&logo=discord

[dependency-link]: https://deps.rs/repo/github/RiceCX/KanesMaidBot
[dependency-badge]: https://deps.rs/repo/github/RiceCX/KanesMaidBot/status.svg

[license-link]: https://github.com/RiceCX/KanesMaidBot/blob/master/LICENSE
[license-badge]: https://img.shields.io/github/license/RiceCX/KanesMaidBot.svg?color=ff1f46&style=flat-square

[github-actions-link]: https://github.com/RiceCX/KanesMaidBot/actions/workflows/ci.yml
[github-actions-badge]: https://github.com/RiceCX/KanesMaidBot/actions/workflows/ci.yml/badge.svg?branch=master

[github-contribution-link]: https://github.com/RiceCX/KanesMaidBot/blob/master/CONTRIBUTING.md

[semantic-versioning-link]: https://semver.org/