# Program for easier command line navigating

![Screenshot2](https://github.com/JikoUnderscore/navigatecl/assets/59426055/0922ddc6-2c47-4064-af0f-bd5730b5093b)

### Compile

`cargo build --profile release-lto`

### Use

run program from `nv.ps1` or `nv.sh` (linux untested and ChatGPT genereted)

### Hotkeys

| Commad | Description | 
|:-------------|:--------------:|
| Shift+Tab(Backtab)            | Move back currend file tree         |
| Tab                           | Move forword currend file tree to last dir went         |
| F1                            | Auto comleate if there is one item         |
| Backspace                     | Delete input         |
| Enter                         | Enter folder         |
| Esc                           | `cd` to curent navigated folder|

---
Set-Alias z C:\Users\Underscore\cl\nv.ps1

---
TODO:  
- [ ] update folder contents  
- [ ] print after deleting letter  
- [ ] fullscreen support  
- [ ] (Windows) change drive letter