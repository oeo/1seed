# 1seed completions for fish

set -l commands pub priv ssh ssh-pub sign-pub enc dec sign verify pw raw mnemonic ssh-add config realms info

# Disable file completions globally, enable per-command
complete -c 1seed -f

# Global options
complete -c 1seed -s r -l realm -d 'Realm' -xa '(1seed realms 2>/dev/null)'
complete -c 1seed -s f -l seed-file -d 'Seed file' -r
complete -c 1seed -s h -l help -d 'Show help'
complete -c 1seed -s V -l version -d 'Show version'

# Commands
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a pub -d 'Age public key'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a priv -d 'Age private key'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a ssh -d 'SSH private key'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a ssh-pub -d 'SSH public key'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a sign-pub -d 'Signing public key'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a enc -d 'Encrypt'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a dec -d 'Decrypt'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a sign -d 'Sign'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a verify -d 'Verify'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a pw -d 'Derive password'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a raw -d 'Derive raw bytes'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a mnemonic -d 'Derive mnemonic'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a ssh-add -d 'Add SSH key to agent'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a config -d 'Configuration'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a realms -d 'Manage realms'
complete -c 1seed -n "not __fish_seen_subcommand_from $commands" -a info -d 'Show status'

# enc options
complete -c 1seed -n "__fish_seen_subcommand_from enc" -s R -l recipient -d 'Recipient'
complete -c 1seed -n "__fish_seen_subcommand_from enc" -s F -l recipients-file -d 'Recipients file' -r
complete -c 1seed -n "__fish_seen_subcommand_from enc" -s s -l self -d 'Include self'
complete -c 1seed -n "__fish_seen_subcommand_from enc" -s p -l passphrase -d 'Use passphrase'
complete -c 1seed -n "__fish_seen_subcommand_from enc" -s a -l armor -d 'ASCII armor'
complete -c 1seed -n "__fish_seen_subcommand_from enc" -s o -l output -d 'Output file' -r

# dec options
complete -c 1seed -n "__fish_seen_subcommand_from dec" -s k -l key -d 'Key file' -r
complete -c 1seed -n "__fish_seen_subcommand_from dec" -s p -l passphrase -d 'Use passphrase'
complete -c 1seed -n "__fish_seen_subcommand_from dec" -s o -l output -d 'Output file' -r

# sign options
complete -c 1seed -n "__fish_seen_subcommand_from sign" -s o -l output -d 'Output file' -r
complete -c 1seed -n "__fish_seen_subcommand_from sign" -l binary -d 'Binary output'

# verify options
complete -c 1seed -n "__fish_seen_subcommand_from verify" -s k -l pubkey -d 'Public key'

# pw options
complete -c 1seed -n "__fish_seen_subcommand_from pw" -s l -l length -d 'Length'
complete -c 1seed -n "__fish_seen_subcommand_from pw" -s n -l counter -d 'Counter'
complete -c 1seed -n "__fish_seen_subcommand_from pw" -l no-symbols -d 'No symbols'
complete -c 1seed -n "__fish_seen_subcommand_from pw" -l symbols -d 'Symbol set'

# raw options
complete -c 1seed -n "__fish_seen_subcommand_from raw" -s l -l length -d 'Length'
complete -c 1seed -n "__fish_seen_subcommand_from raw" -l hex -d 'Hex output'
complete -c 1seed -n "__fish_seen_subcommand_from raw" -l base64 -d 'Base64 output'
complete -c 1seed -n "__fish_seen_subcommand_from raw" -l binary -d 'Binary output'

# mnemonic options
complete -c 1seed -n "__fish_seen_subcommand_from mnemonic" -s w -l words -d 'Word count' -xa '12 15 18 21 24'

# ssh-add options
complete -c 1seed -n "__fish_seen_subcommand_from ssh-add" -s t -l lifetime -d 'Lifetime'
complete -c 1seed -n "__fish_seen_subcommand_from ssh-add" -s c -l confirm -d 'Confirm'

# config subcommands
complete -c 1seed -n "__fish_seen_subcommand_from config; and not __fish_seen_subcommand_from set get path" -a set -d 'Set value'
complete -c 1seed -n "__fish_seen_subcommand_from config; and not __fish_seen_subcommand_from set get path" -a get -d 'Get value'
complete -c 1seed -n "__fish_seen_subcommand_from config; and not __fish_seen_subcommand_from set get path" -a path -d 'Show path'
complete -c 1seed -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from set get" -a 'realm seed-file'

# realms subcommands
complete -c 1seed -n "__fish_seen_subcommand_from realms; and not __fish_seen_subcommand_from add rm" -a add -d 'Add realm'
complete -c 1seed -n "__fish_seen_subcommand_from realms; and not __fish_seen_subcommand_from add rm" -a rm -d 'Remove realm'
complete -c 1seed -n "__fish_seen_subcommand_from realms; and __fish_seen_subcommand_from rm" -xa '(1seed realms 2>/dev/null)'
