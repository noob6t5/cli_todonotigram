# cli_todonotigram 📋
A CLI tool for managing to-do lists, directly from terminal.

I will add more features integrated with a Telegram bot for real-time task notifications and updates.

<img src="https://github.com/noob6t5/cli_todonotigram/blob/main/icon/icon.png" width="200" height="200" />

# Insp
I keep forgetting thing's like to scan ,to monitor & making changes in file directories . that's why this i created this . It might also help you wdireclty from termianl without any hassle.


#  Features

- Simple task management directly in your terminal
- Planned Telegram integration for instant updates
- Lightweight and fast CLI performance

![features](https://github.com/user-attachments/assets/bc1c612c-73b1-4b81-a3fd-46e622bfa817)


# Configuration's  🛠️ Setup Instructions

1. Cloning the Repo 
2. Compiling it
3. moving todo to bin
4. Creating a script that runs todo list at startup.

### Prerequisites
Ensure that **Rust** is installed on your system before proceeding.

### Installation Steps

1. **Clone the Repository:**
   ```
   git clone https://github.com/noob6t5/cli_todonotigram.git
   cd cli_todonotigram
   ```
2.  **Build:**
    ```
    cargo build --release
    ```

3. **Move the Binary to Your System PATH: After building, the compiled binary will be in target/release/todo.**
    ```
    sudo mv target/release/todo /usr/local/bin/
    ```
 

Then You can just type `todo`  to run it widely from anywhere in terminal..

![basic](https://github.com/user-attachments/assets/908ab75b-7c91-4eb8-9357-6813ef208632)



# If you want it to show it  everytime when you Open Device then you can write script to run it in startup

# 🚀 Add to Startup

Make a script named any let's say `show_todo_once.sh` .

```
#!/bin/bash

FLAG_FILE="$HOME/.todo_shown"
if [ ! -f "$FLAG_FILE" ]; then
    todo list
    # Create the flag file to indicate the script has run
    touch "$FLAG_FILE"
fi
```
Save it Now `chmod +x ~/show_todo_once.sh` to make it executable.

Then add this `~/show_todo_once.sh`   accodring to your shell in ~/.zshrc or ~/.bashrc.

## 🎉 Future Enhancements
  Stay tuned for Telegram integration and more advanced features. Star the repo to stay updated!


