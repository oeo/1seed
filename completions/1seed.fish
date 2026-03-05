# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_1seed_global_optspecs
	string join \n realm= h/help V/version
end

function __fish_1seed_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_1seed_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_1seed_using_subcommand
	set -l cmd (__fish_1seed_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c 1seed -n "__fish_1seed_needs_command" -l realm -r
complete -c 1seed -n "__fish_1seed_needs_command" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_needs_command" -s V -l version -d 'Print version'
complete -c 1seed -n "__fish_1seed_needs_command" -f -a "age" -d 'Age encryption keys and operations'
complete -c 1seed -n "__fish_1seed_needs_command" -f -a "ssh" -d 'SSH keys and operations'
complete -c 1seed -n "__fish_1seed_needs_command" -f -a "sign" -d 'Ed25519 signing keys and operations'
complete -c 1seed -n "__fish_1seed_needs_command" -f -a "derive" -d 'Derive passwords, mnemonics, and raw bytes'
complete -c 1seed -n "__fish_1seed_needs_command" -f -a "init" -d 'Initialize: store seed in OS keychain'
complete -c 1seed -n "__fish_1seed_needs_command" -f -a "forget" -d 'Remove seed from OS keychain'
complete -c 1seed -n "__fish_1seed_needs_command" -f -a "status" -d 'Show status and derived keys'
complete -c 1seed -n "__fish_1seed_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and not __fish_seen_subcommand_from pub key encrypt decrypt help" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand age; and not __fish_seen_subcommand_from pub key encrypt decrypt help" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and not __fish_seen_subcommand_from pub key encrypt decrypt help" -f -a "pub" -d 'Show age public key'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and not __fish_seen_subcommand_from pub key encrypt decrypt help" -f -a "key" -d 'Show age private key'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and not __fish_seen_subcommand_from pub key encrypt decrypt help" -f -a "encrypt" -d 'Encrypt file with age'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and not __fish_seen_subcommand_from pub key encrypt decrypt help" -f -a "decrypt" -d 'Decrypt file with age'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and not __fish_seen_subcommand_from pub key encrypt decrypt help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from pub" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from pub" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from key" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from key" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from encrypt" -s R -l recipient -r
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from encrypt" -s F -l recipients-file -r -F
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from encrypt" -s o -l output -r -F
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from encrypt" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from encrypt" -s s -l self
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from encrypt" -s p -l passphrase
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from encrypt" -s a -l armor
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from encrypt" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from decrypt" -s k -l key -r -F
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from decrypt" -s o -l output -r -F
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from decrypt" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from decrypt" -s p -l passphrase
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from decrypt" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from help" -f -a "pub" -d 'Show age public key'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from help" -f -a "key" -d 'Show age private key'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from help" -f -a "encrypt" -d 'Encrypt file with age'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from help" -f -a "decrypt" -d 'Decrypt file with age'
complete -c 1seed -n "__fish_1seed_using_subcommand age; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and not __fish_seen_subcommand_from pub key add help" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and not __fish_seen_subcommand_from pub key add help" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and not __fish_seen_subcommand_from pub key add help" -f -a "pub" -d 'Show SSH public key'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and not __fish_seen_subcommand_from pub key add help" -f -a "key" -d 'Show SSH private key'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and not __fish_seen_subcommand_from pub key add help" -f -a "add" -d 'Add SSH key to agent'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and not __fish_seen_subcommand_from pub key add help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from pub" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from pub" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from key" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from key" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from add" -s l -l lifetime -r
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from add" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from add" -s c -l confirm
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from help" -f -a "pub" -d 'Show SSH public key'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from help" -f -a "key" -d 'Show SSH private key'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from help" -f -a "add" -d 'Add SSH key to agent'
complete -c 1seed -n "__fish_1seed_using_subcommand ssh; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and not __fish_seen_subcommand_from pub data verify help" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and not __fish_seen_subcommand_from pub data verify help" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and not __fish_seen_subcommand_from pub data verify help" -f -a "pub" -d 'Show signing public key'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and not __fish_seen_subcommand_from pub data verify help" -f -a "data" -d 'Sign data'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and not __fish_seen_subcommand_from pub data verify help" -f -a "verify" -d 'Verify signature'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and not __fish_seen_subcommand_from pub data verify help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from pub" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from pub" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from data" -s o -l output -r -F
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from data" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from data" -l binary
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from data" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from verify" -s k -l pubkey -r
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from verify" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from verify" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from help" -f -a "pub" -d 'Show signing public key'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from help" -f -a "data" -d 'Sign data'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from help" -f -a "verify" -d 'Verify signature'
complete -c 1seed -n "__fish_1seed_using_subcommand sign; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and not __fish_seen_subcommand_from password mnemonic int uuid raw help" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and not __fish_seen_subcommand_from password mnemonic int uuid raw help" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and not __fish_seen_subcommand_from password mnemonic int uuid raw help" -f -a "password" -d 'Derive password for site'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and not __fish_seen_subcommand_from password mnemonic int uuid raw help" -f -a "mnemonic" -d 'Derive BIP39 mnemonic'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and not __fish_seen_subcommand_from password mnemonic int uuid raw help" -f -a "int" -d 'Derive uniform integer'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and not __fish_seen_subcommand_from password mnemonic int uuid raw help" -f -a "uuid" -d 'Derive UUID (v4-compatible)'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and not __fish_seen_subcommand_from password mnemonic int uuid raw help" -f -a "raw" -d 'Derive raw bytes'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and not __fish_seen_subcommand_from password mnemonic int uuid raw help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from password" -s l -l length -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from password" -l symbols -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from password" -s n -l counter -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from password" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from password" -l no-symbols
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from password" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from mnemonic" -s w -l words -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from mnemonic" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from mnemonic" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from int" -l min -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from int" -l max -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from int" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from int" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from uuid" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from uuid" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from raw" -s l -l length -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from raw" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from raw" -l hex
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from raw" -l base64
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from raw" -l binary
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from raw" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from help" -f -a "password" -d 'Derive password for site'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from help" -f -a "mnemonic" -d 'Derive BIP39 mnemonic'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from help" -f -a "int" -d 'Derive uniform integer'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from help" -f -a "uuid" -d 'Derive UUID (v4-compatible)'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from help" -f -a "raw" -d 'Derive raw bytes'
complete -c 1seed -n "__fish_1seed_using_subcommand derive; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand init" -l from-file -r -F
complete -c 1seed -n "__fish_1seed_using_subcommand init" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand init" -s p -l passphrase
complete -c 1seed -n "__fish_1seed_using_subcommand init" -s g -l generate
complete -c 1seed -n "__fish_1seed_using_subcommand init" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand forget" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand forget" -l confirm
complete -c 1seed -n "__fish_1seed_using_subcommand forget" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand status" -l realm -r
complete -c 1seed -n "__fish_1seed_using_subcommand status" -s h -l help -d 'Print help'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and not __fish_seen_subcommand_from age ssh sign derive init forget status help" -f -a "age" -d 'Age encryption keys and operations'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and not __fish_seen_subcommand_from age ssh sign derive init forget status help" -f -a "ssh" -d 'SSH keys and operations'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and not __fish_seen_subcommand_from age ssh sign derive init forget status help" -f -a "sign" -d 'Ed25519 signing keys and operations'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and not __fish_seen_subcommand_from age ssh sign derive init forget status help" -f -a "derive" -d 'Derive passwords, mnemonics, and raw bytes'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and not __fish_seen_subcommand_from age ssh sign derive init forget status help" -f -a "init" -d 'Initialize: store seed in OS keychain'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and not __fish_seen_subcommand_from age ssh sign derive init forget status help" -f -a "forget" -d 'Remove seed from OS keychain'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and not __fish_seen_subcommand_from age ssh sign derive init forget status help" -f -a "status" -d 'Show status and derived keys'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and not __fish_seen_subcommand_from age ssh sign derive init forget status help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from age" -f -a "pub" -d 'Show age public key'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from age" -f -a "key" -d 'Show age private key'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from age" -f -a "encrypt" -d 'Encrypt file with age'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from age" -f -a "decrypt" -d 'Decrypt file with age'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from ssh" -f -a "pub" -d 'Show SSH public key'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from ssh" -f -a "key" -d 'Show SSH private key'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from ssh" -f -a "add" -d 'Add SSH key to agent'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from sign" -f -a "pub" -d 'Show signing public key'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from sign" -f -a "data" -d 'Sign data'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from sign" -f -a "verify" -d 'Verify signature'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from derive" -f -a "password" -d 'Derive password for site'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from derive" -f -a "mnemonic" -d 'Derive BIP39 mnemonic'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from derive" -f -a "int" -d 'Derive uniform integer'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from derive" -f -a "uuid" -d 'Derive UUID (v4-compatible)'
complete -c 1seed -n "__fish_1seed_using_subcommand help; and __fish_seen_subcommand_from derive" -f -a "raw" -d 'Derive raw bytes'
