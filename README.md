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

## 💾 Install

Currently, `mathing` is only availabe via cloning on github and building from source via `cargo`. There are plans to (at least) package `mathing` for Nix/NixOs.

## 👐 Usage

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
