# 🎲 numba-wumbo

**numba-wumbo** is a goofy little terminal game where you try to guess the secret number between **1 and 1000**.  
It’s simple, fun, and brings RNG magic straight to your CLI.

---

## 🚀 Installation

### From [crates.io](https://crates.io/crates/numba-wumbo)

```bash
cargo install numba-wumbo
```

This will make the `numba-wumbo` command available globally.

---

## 🎮 How to Play

Run the game in your terminal:

```bash
numba-wumbo
```

You’ll see a simple menu:

```
🎲 Welcome to numba-wumbo! 🎲
1. Play the game
2. About
3. Quit
```

- Choose **1** to start guessing a random number between 1 and 1000.
- You’ll get feedback if your guess is **too high** or **too low** until you nail the correct number.
- Choose **2** to see game info.
- Choose **3** to quit.

---

## 📸 Example Gameplay

```
🎲 Welcome to numba-wumbo! 🎲
1. Play the game
2. About
3. Quit
Enter your choice: 1

Enter your guess (1-1000):
500
Too small! Try again.

750
Too big! Try again.

694
🎉 You guessed it! The secret number was 694!
```

---

## 📦 Development

Clone and run locally:

```bash
git clone https://github.com/sudosuanjal/numba-wumbo
cd numba-wumbo
cargo run
```

Format & lint before committing:

```bash
cargo fmt
cargo clippy
```

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!  
Feel free to fork the repo and open a PR.

---

## 👤 Author

**Anjal**  
🐦 [@sudosuanjal](https://x.com/sudosuanjal)

---

## 📝 License

This project is licensed under the [MIT License](LICENSE).
