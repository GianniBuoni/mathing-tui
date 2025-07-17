# MATHING!

<!--toc:start-->

- [💸 Motivation](#-motivation)
- [💾 Install](#-install)
- [👐 Usage](#-usage)
- [🔨 Configuration Files](#-configuration-files)
- [💻 Keymap Configurations](#-keymap-configurations)
<!--toc:end-->

---

## 💸 Motivation

Expense splitting in the terminal! Woohoo!

Often grocery expenses are pretty consistent between me and my roomate. Week to week we usually end up getting the same staple items, and for the most part, quantities are variable. This was a exercise in rolling up a system that took that into account, by storing the various items we get, and perform the calculations for me.

As someone who likes living in the terminal, a TUI was a good vehicle for making something that I could lean a lot from while making, and for making something I would use often. **Key concepts I explored in building this app include**:

- SQLite databases and database testing using the [sqlx crate](https://github.com/launchbadge/sqlx)
- Keymap configuration
- Hand rolling an API for sending Requests to the database and for handling the responses
- Input handling
- Concurrent programming/learning async Rust.

## 💾 Install

Currently, `mathing` is only availabe via cloning on github and building from source via `cargo`. There are plans to (at least) package `mathing` for Nix/NixOs.

## 👐 Usage

Running `mathing` will pull up the main TUI. There are three tables the `Items`, `Receipts`, `Users`. By default, navigating is handled by tabbing through the available tables. Adding/deleting/updating items to/in the tables puts the app into an "insert mode" and allows the user to ues the keyboard to fill out a form related to the action performed.

Form validation errors will prevent the form from submitting and will displays the error. Other errors are displayed in a dialogue box popup.

For a full list of commands, see the [keymap configuration](#-keymap-configurations) section for the full list of available actions.

## 🔨 Configuration Files

The configuration directory stores all the data needed for `mathing` to run.  
This includes:

- `keymap.toml` : configuration file to map keystrokes to app actions.
- `data.db`: sqlite database.

By default, configuration files live in the `.config/mathing` directory on Mac and Linux.

- Configuration file location can be changed by setting the `MATHING_CONFIG` varaible.
- Run `mathing --help` to check where your config files are currently set to.

## 💻 Keymap Configurations

`keymap.toml` is a file of _key event_ -> _command string_ pairs. By default, if `mathing` is run without a keymap file configured/present, the default keymap will be wirtten in the config directory.

Keys events consist of a key and a single otional modifer.  
Supported modifiers are:

- SHIFT
- CRTL
- ALT

`mathing` uses `crossterm` as its key event backent. You can read more about the key events at the [crossterm github page.](https://github.com/crossterm-rs/crossterm).

The following commands can be configured:

| Command        | Purpose                                                  |
| -------------- | -------------------------------------------------------- |
| AddToReceipt   | Add the current Store Item to the current Receipt table. |
| DeleteSelected | Delete active table's currently selected item.           |
| EditSelected   | Edit active table's currently selected item.             |
| EnterInsert    | Add new item to the active table.                        |
| EnterNormal    | Cancel current form/action.                              |
| MakeSelection  | For muti-select forms: add active choice to selection.   |
| NavigateLeft   | Go to active table's next page.                          |
| NavigateDown   | Select active table's next item.                         |
| NavigateUp     | Select active table's previous item.                     |
| NavigateRight  | Go to active table's previous page.                      |
| Refresh        | Refetch all data from the database.                      |
| Search         | Search for a Store Item.                                 |
| SelectForward  | Select/activate the next table.                          |
| SelectBackward | Select/activate the previous table.                      |
| Submit         | For forms: submit current form.                          |
| Help           | Show current key mappings and config directory location. |
