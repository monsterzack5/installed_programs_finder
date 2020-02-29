A program to search your ~/.zsh_history file for all programs you've installed
with 'apt install', remove the programs you've removed with 'apt remove' or
'apt purge', verify they're installed with 'apt -qq list <program>' and then
save those programs to a file called 'installed-programs.log'.

usage: cargo run ~/.zsh_history
               or
./installed_program_finder ~/.zsh_history

Measured as using negligible CPU and 580KB of ram, with a stripped binary of
1.5MB (Nonstripped being 4.0MB).

first rust project!