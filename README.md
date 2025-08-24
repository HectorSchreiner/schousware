# Shousware C2 Server

# Roadmap
- [x] Init
- [x] Simple TUI
- [x] Local Text Config
- [x] Simple fileserver
- [ ] Create Simple payload
- [ ] Recieve Status from machines
- [x] Send commands (ikke testet, men tror det virker)
- [ ] Replace Text Config with local sql server

# Installation
You can clone the project and run the `Debug Build` it with 
```
cargo run
```

# Screenshots
<img title="Infected Menu" src="/screenshots/infected_menu.png">
<img title="Stats Menu" src="/screenshots/stats_menu.png">


# Tools

### C4
C4 is a Command & Control framework, build specifically for the Schousware suite. It integrates with the buildin malware tools, such as Weirware and the infamous Buff3_Trojan

### Weirware
Weirware is a simple included ransomware tool, built for the C4 server.
