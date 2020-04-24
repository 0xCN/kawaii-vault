# Kawaii Vault (づ｡◕‿‿◕｡)づ

## Setup
You need to fill in Config.toml with the required info.
```toml
id = "(gist-id)"
access_token = "GitHub Token (that has access to gist)"

[files]
vaultd = "(gist-vaultd-filename)"
unique = "(gist-unique-filename)"
```
Here is an [example gist](https://gist.github.com/0xCN/bd758a3bfacfba6917a5c5badcfe9004).


## Initialization
```bash
./kawaii-vault initialize
```
The **password** you choose in this step will be used in decryption.


## Usage
on editor mode
```
CTRL-S: Save
CTRL-Q: Quit
CTRL-F: Find
CTRL-P: Generate a Password
```
![](kawaii-vault.png)
