# Discord Music Bot in Rust 🎵

<div align="center">
  <img src="./assets/rustyBot.png" alt="Rusty Bot Logo" width="200">
</div>


A simple yet functional Discord bot written in Rust that can play music from YouTube in voice channels. Built using the Serenity and Songbird libraries.

## Features ✨

- Join voice channels (`!join`)
- Play music from YouTube URLs (`!play`)
- Leave voice channel (`!leave`)
- Automatic volume control
- Status messages with emojis
- Robust error handling

## Prerequisites 📋

Before running the bot, make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp#installation)
- [FFmpeg](https://ffmpeg.org/download.html)

### macOS
```bash
# Using Homebrew
brew install rust
brew install yt-dlp
brew install ffmpeg
```

### Linux (Ubuntu/Debian)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install yt-dlp and ffmpeg
sudo apt update
sudo apt install yt-dlp ffmpeg
```

## Setup 🔧

1. Create a new application in the [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a bot for your application and copy the token
3. Replace `"PLACE_YOUR_TOKEN_HERE"` in `src/main.rs` with your token
4. Invite the bot to your server using the OAuth2 URL generated in the portal

## Installation 🚀

```bash
# Clone the repository
git clone git@github.com:louire/discord_plays.git
cd discord_plays

# Build and run
cargo run
```

## Usage 📖

Once the bot is online, you can use the following commands:

- `!join` - Bot joins your current voice channel
- `!play <url>` - Plays audio from a YouTube video
- `!leave` - Bot leaves the voice channel

Example:
```
!play https://www.youtube.com/watch?v=dQw4w9WgXcQ
```

## Troubleshooting 🔍

If you encounter issues:

1. Verify all dependencies are installed:
```bash
yt-dlp --version
ffmpeg -version
```

2. Ensure the bot has the necessary Discord permissions:
   - Connect (to voice channels)
   - Speak
   - Send Messages

3. Check that the YouTube URL is valid and accessible

4. Review console logs for error messages

## Contributing 🤝

Contributions are welcome! If you find a bug or have a suggestion, please open an issue or submit a pull request.

## License 📄

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments 👏

- [Serenity](https://github.com/serenity-rs/serenity) - Discord framework for Rust
- [Songbird](https://github.com/serenity-rs/songbird) - Voice library for Serenity
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - YouTube downloader