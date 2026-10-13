#compdef 1seed

autoload -U is-at-least

_1seed() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'--seed-file=[]:SEED_FILE:_files' \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_1seed_commands" \
"*::: :->1seed" \
&& ret=0
    case $state in
    (1seed)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-command-$line[1]:"
        case $line[1] in
            (age)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
":: :_1seed__subcmd__age_commands" \
"*::: :->age" \
&& ret=0

    case $state in
    (age)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-age-command-$line[1]:"
        case $line[1] in
            (pub)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(key)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(encrypt)
_arguments "${_arguments_options[@]}" : \
'*-R+[]:RECIPIENTS:_default' \
'*--recipient=[]:RECIPIENTS:_default' \
'*-F+[]:RECIPIENT_FILES:_files' \
'*--recipients-file=[]:RECIPIENT_FILES:_files' \
'-o+[]:OUTPUT:_files' \
'--output=[]:OUTPUT:_files' \
'--realm=[]:REALM:_default' \
'-s[]' \
'--self[]' \
'-p[]' \
'--passphrase[]' \
'-a[]' \
'--armor[]' \
'-h[Print help]' \
'--help[Print help]' \
'::file:_files' \
&& ret=0
;;
(decrypt)
_arguments "${_arguments_options[@]}" : \
'-k+[]:KEY:_files' \
'--key=[]:KEY:_files' \
'-o+[]:OUTPUT:_files' \
'--output=[]:OUTPUT:_files' \
'--realm=[]:REALM:_default' \
'-p[]' \
'--passphrase[]' \
'-h[Print help]' \
'--help[Print help]' \
'::file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_1seed__subcmd__age__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-age-help-command-$line[1]:"
        case $line[1] in
            (pub)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(key)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(encrypt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(decrypt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(ssh)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
":: :_1seed__subcmd__ssh_commands" \
"*::: :->ssh" \
&& ret=0

    case $state in
    (ssh)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-ssh-command-$line[1]:"
        case $line[1] in
            (pub)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(key)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
'-l+[]:LIFETIME:_default' \
'--lifetime=[]:LIFETIME:_default' \
'--realm=[]:REALM:_default' \
'-c[]' \
'--confirm[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_1seed__subcmd__ssh__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-ssh-help-command-$line[1]:"
        case $line[1] in
            (pub)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(key)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(sign)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
":: :_1seed__subcmd__sign_commands" \
"*::: :->sign" \
&& ret=0

    case $state in
    (sign)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-sign-command-$line[1]:"
        case $line[1] in
            (pub)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(data)
_arguments "${_arguments_options[@]}" : \
'-o+[]:OUTPUT:_files' \
'--output=[]:OUTPUT:_files' \
'--realm=[]:REALM:_default' \
'--binary[]' \
'-h[Print help]' \
'--help[Print help]' \
'::file:_files' \
&& ret=0
;;
(verify)
_arguments "${_arguments_options[@]}" : \
'-k+[]:PUBKEY:_default' \
'--pubkey=[]:PUBKEY:_default' \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
':signature -- Signature (base64, or @file):_default' \
'::file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_1seed__subcmd__sign__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-sign-help-command-$line[1]:"
        case $line[1] in
            (pub)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(data)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(verify)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(derive)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
":: :_1seed__subcmd__derive_commands" \
"*::: :->derive" \
&& ret=0

    case $state in
    (derive)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-derive-command-$line[1]:"
        case $line[1] in
            (password)
_arguments "${_arguments_options[@]}" : \
'-l+[]:LENGTH:_default' \
'--length=[]:LENGTH:_default' \
'--symbols=[]:SYMBOLS:_default' \
'-n+[]:COUNTER:_default' \
'--counter=[]:COUNTER:_default' \
'--realm=[]:REALM:_default' \
'--no-symbols[]' \
'-h[Print help]' \
'--help[Print help]' \
':site:_default' \
&& ret=0
;;
(mnemonic)
_arguments "${_arguments_options[@]}" : \
'-w+[]:WORDS:_default' \
'--words=[]:WORDS:_default' \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(int)
_arguments "${_arguments_options[@]}" : \
'--min=[]:MIN:_default' \
'--max=[]:MAX:_default' \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
':path:_default' \
&& ret=0
;;
(uuid)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
':path:_default' \
&& ret=0
;;
(raw)
_arguments "${_arguments_options[@]}" : \
'-l+[]:LENGTH:_default' \
'--length=[]:LENGTH:_default' \
'--realm=[]:REALM:_default' \
'--hex[]' \
'--base64[]' \
'--binary[]' \
'-h[Print help]' \
'--help[Print help]' \
':path:_default' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_1seed__subcmd__derive__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-derive-help-command-$line[1]:"
        case $line[1] in
            (password)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(mnemonic)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(int)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(uuid)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(raw)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(init)
_arguments "${_arguments_options[@]}" : \
'--from-file=[]:FROM_FILE:_files' \
'--realm=[]:REALM:_default' \
'-p[]' \
'--passphrase[]' \
'-g[]' \
'--generate[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(forget)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'--confirm[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'--realm=[]:REALM:_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_1seed__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-help-command-$line[1]:"
        case $line[1] in
            (age)
_arguments "${_arguments_options[@]}" : \
":: :_1seed__subcmd__help__subcmd__age_commands" \
"*::: :->age" \
&& ret=0

    case $state in
    (age)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-help-age-command-$line[1]:"
        case $line[1] in
            (pub)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(key)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(encrypt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(decrypt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(ssh)
_arguments "${_arguments_options[@]}" : \
":: :_1seed__subcmd__help__subcmd__ssh_commands" \
"*::: :->ssh" \
&& ret=0

    case $state in
    (ssh)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-help-ssh-command-$line[1]:"
        case $line[1] in
            (pub)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(key)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(sign)
_arguments "${_arguments_options[@]}" : \
":: :_1seed__subcmd__help__subcmd__sign_commands" \
"*::: :->sign" \
&& ret=0

    case $state in
    (sign)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-help-sign-command-$line[1]:"
        case $line[1] in
            (pub)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(data)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(verify)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(derive)
_arguments "${_arguments_options[@]}" : \
":: :_1seed__subcmd__help__subcmd__derive_commands" \
"*::: :->derive" \
&& ret=0

    case $state in
    (derive)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:1seed-help-derive-command-$line[1]:"
        case $line[1] in
            (password)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(mnemonic)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(int)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(uuid)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(raw)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(init)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(forget)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_1seed_commands] )) ||
_1seed_commands() {
    local commands; commands=(
'age:Age encryption keys and operations' \
'ssh:SSH keys and operations' \
'sign:Ed25519 signing keys and operations' \
'derive:Derive passwords, mnemonics, and raw bytes' \
'init:Initialize\: store seed in OS keychain (~/.1seed on Linux)' \
'forget:Remove seed from OS keychain and ~/.1seed' \
'status:Show status and derived keys' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age_commands] )) ||
_1seed__subcmd__age_commands() {
    local commands; commands=(
'pub:Show age public key' \
'key:Show age private key' \
'encrypt:Encrypt file with age' \
'decrypt:Decrypt file with age' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed age commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__decrypt_commands] )) ||
_1seed__subcmd__age__subcmd__decrypt_commands() {
    local commands; commands=()
    _describe -t commands '1seed age decrypt commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__encrypt_commands] )) ||
_1seed__subcmd__age__subcmd__encrypt_commands() {
    local commands; commands=()
    _describe -t commands '1seed age encrypt commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__help_commands] )) ||
_1seed__subcmd__age__subcmd__help_commands() {
    local commands; commands=(
'pub:Show age public key' \
'key:Show age private key' \
'encrypt:Encrypt file with age' \
'decrypt:Decrypt file with age' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed age help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__help__subcmd__decrypt_commands] )) ||
_1seed__subcmd__age__subcmd__help__subcmd__decrypt_commands() {
    local commands; commands=()
    _describe -t commands '1seed age help decrypt commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__help__subcmd__encrypt_commands] )) ||
_1seed__subcmd__age__subcmd__help__subcmd__encrypt_commands() {
    local commands; commands=()
    _describe -t commands '1seed age help encrypt commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__help__subcmd__help_commands] )) ||
_1seed__subcmd__age__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands '1seed age help help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__help__subcmd__key_commands] )) ||
_1seed__subcmd__age__subcmd__help__subcmd__key_commands() {
    local commands; commands=()
    _describe -t commands '1seed age help key commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__help__subcmd__pub_commands] )) ||
_1seed__subcmd__age__subcmd__help__subcmd__pub_commands() {
    local commands; commands=()
    _describe -t commands '1seed age help pub commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__key_commands] )) ||
_1seed__subcmd__age__subcmd__key_commands() {
    local commands; commands=()
    _describe -t commands '1seed age key commands' commands "$@"
}
(( $+functions[_1seed__subcmd__age__subcmd__pub_commands] )) ||
_1seed__subcmd__age__subcmd__pub_commands() {
    local commands; commands=()
    _describe -t commands '1seed age pub commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive_commands] )) ||
_1seed__subcmd__derive_commands() {
    local commands; commands=(
'password:Derive password for site' \
'mnemonic:Derive BIP39 mnemonic' \
'int:Derive uniform integer' \
'uuid:Derive UUID (v4-compatible)' \
'raw:Derive raw bytes' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed derive commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__help_commands] )) ||
_1seed__subcmd__derive__subcmd__help_commands() {
    local commands; commands=(
'password:Derive password for site' \
'mnemonic:Derive BIP39 mnemonic' \
'int:Derive uniform integer' \
'uuid:Derive UUID (v4-compatible)' \
'raw:Derive raw bytes' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed derive help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__help__subcmd__help_commands] )) ||
_1seed__subcmd__derive__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive help help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__help__subcmd__int_commands] )) ||
_1seed__subcmd__derive__subcmd__help__subcmd__int_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive help int commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__help__subcmd__mnemonic_commands] )) ||
_1seed__subcmd__derive__subcmd__help__subcmd__mnemonic_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive help mnemonic commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__help__subcmd__password_commands] )) ||
_1seed__subcmd__derive__subcmd__help__subcmd__password_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive help password commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__help__subcmd__raw_commands] )) ||
_1seed__subcmd__derive__subcmd__help__subcmd__raw_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive help raw commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__help__subcmd__uuid_commands] )) ||
_1seed__subcmd__derive__subcmd__help__subcmd__uuid_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive help uuid commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__int_commands] )) ||
_1seed__subcmd__derive__subcmd__int_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive int commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__mnemonic_commands] )) ||
_1seed__subcmd__derive__subcmd__mnemonic_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive mnemonic commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__password_commands] )) ||
_1seed__subcmd__derive__subcmd__password_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive password commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__raw_commands] )) ||
_1seed__subcmd__derive__subcmd__raw_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive raw commands' commands "$@"
}
(( $+functions[_1seed__subcmd__derive__subcmd__uuid_commands] )) ||
_1seed__subcmd__derive__subcmd__uuid_commands() {
    local commands; commands=()
    _describe -t commands '1seed derive uuid commands' commands "$@"
}
(( $+functions[_1seed__subcmd__forget_commands] )) ||
_1seed__subcmd__forget_commands() {
    local commands; commands=()
    _describe -t commands '1seed forget commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help_commands] )) ||
_1seed__subcmd__help_commands() {
    local commands; commands=(
'age:Age encryption keys and operations' \
'ssh:SSH keys and operations' \
'sign:Ed25519 signing keys and operations' \
'derive:Derive passwords, mnemonics, and raw bytes' \
'init:Initialize\: store seed in OS keychain (~/.1seed on Linux)' \
'forget:Remove seed from OS keychain and ~/.1seed' \
'status:Show status and derived keys' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__age_commands] )) ||
_1seed__subcmd__help__subcmd__age_commands() {
    local commands; commands=(
'pub:Show age public key' \
'key:Show age private key' \
'encrypt:Encrypt file with age' \
'decrypt:Decrypt file with age' \
    )
    _describe -t commands '1seed help age commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__age__subcmd__decrypt_commands] )) ||
_1seed__subcmd__help__subcmd__age__subcmd__decrypt_commands() {
    local commands; commands=()
    _describe -t commands '1seed help age decrypt commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__age__subcmd__encrypt_commands] )) ||
_1seed__subcmd__help__subcmd__age__subcmd__encrypt_commands() {
    local commands; commands=()
    _describe -t commands '1seed help age encrypt commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__age__subcmd__key_commands] )) ||
_1seed__subcmd__help__subcmd__age__subcmd__key_commands() {
    local commands; commands=()
    _describe -t commands '1seed help age key commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__age__subcmd__pub_commands] )) ||
_1seed__subcmd__help__subcmd__age__subcmd__pub_commands() {
    local commands; commands=()
    _describe -t commands '1seed help age pub commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__derive_commands] )) ||
_1seed__subcmd__help__subcmd__derive_commands() {
    local commands; commands=(
'password:Derive password for site' \
'mnemonic:Derive BIP39 mnemonic' \
'int:Derive uniform integer' \
'uuid:Derive UUID (v4-compatible)' \
'raw:Derive raw bytes' \
    )
    _describe -t commands '1seed help derive commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__derive__subcmd__int_commands] )) ||
_1seed__subcmd__help__subcmd__derive__subcmd__int_commands() {
    local commands; commands=()
    _describe -t commands '1seed help derive int commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__derive__subcmd__mnemonic_commands] )) ||
_1seed__subcmd__help__subcmd__derive__subcmd__mnemonic_commands() {
    local commands; commands=()
    _describe -t commands '1seed help derive mnemonic commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__derive__subcmd__password_commands] )) ||
_1seed__subcmd__help__subcmd__derive__subcmd__password_commands() {
    local commands; commands=()
    _describe -t commands '1seed help derive password commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__derive__subcmd__raw_commands] )) ||
_1seed__subcmd__help__subcmd__derive__subcmd__raw_commands() {
    local commands; commands=()
    _describe -t commands '1seed help derive raw commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__derive__subcmd__uuid_commands] )) ||
_1seed__subcmd__help__subcmd__derive__subcmd__uuid_commands() {
    local commands; commands=()
    _describe -t commands '1seed help derive uuid commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__forget_commands] )) ||
_1seed__subcmd__help__subcmd__forget_commands() {
    local commands; commands=()
    _describe -t commands '1seed help forget commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__help_commands] )) ||
_1seed__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands '1seed help help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__init_commands] )) ||
_1seed__subcmd__help__subcmd__init_commands() {
    local commands; commands=()
    _describe -t commands '1seed help init commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__sign_commands] )) ||
_1seed__subcmd__help__subcmd__sign_commands() {
    local commands; commands=(
'pub:Show signing public key' \
'data:Sign data' \
'verify:Verify signature' \
    )
    _describe -t commands '1seed help sign commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__sign__subcmd__data_commands] )) ||
_1seed__subcmd__help__subcmd__sign__subcmd__data_commands() {
    local commands; commands=()
    _describe -t commands '1seed help sign data commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__sign__subcmd__pub_commands] )) ||
_1seed__subcmd__help__subcmd__sign__subcmd__pub_commands() {
    local commands; commands=()
    _describe -t commands '1seed help sign pub commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__sign__subcmd__verify_commands] )) ||
_1seed__subcmd__help__subcmd__sign__subcmd__verify_commands() {
    local commands; commands=()
    _describe -t commands '1seed help sign verify commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__ssh_commands] )) ||
_1seed__subcmd__help__subcmd__ssh_commands() {
    local commands; commands=(
'pub:Show SSH public key' \
'key:Show SSH private key' \
'add:Add SSH key to agent' \
    )
    _describe -t commands '1seed help ssh commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__ssh__subcmd__add_commands] )) ||
_1seed__subcmd__help__subcmd__ssh__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands '1seed help ssh add commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__ssh__subcmd__key_commands] )) ||
_1seed__subcmd__help__subcmd__ssh__subcmd__key_commands() {
    local commands; commands=()
    _describe -t commands '1seed help ssh key commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__ssh__subcmd__pub_commands] )) ||
_1seed__subcmd__help__subcmd__ssh__subcmd__pub_commands() {
    local commands; commands=()
    _describe -t commands '1seed help ssh pub commands' commands "$@"
}
(( $+functions[_1seed__subcmd__help__subcmd__status_commands] )) ||
_1seed__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands '1seed help status commands' commands "$@"
}
(( $+functions[_1seed__subcmd__init_commands] )) ||
_1seed__subcmd__init_commands() {
    local commands; commands=()
    _describe -t commands '1seed init commands' commands "$@"
}
(( $+functions[_1seed__subcmd__sign_commands] )) ||
_1seed__subcmd__sign_commands() {
    local commands; commands=(
'pub:Show signing public key' \
'data:Sign data' \
'verify:Verify signature' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed sign commands' commands "$@"
}
(( $+functions[_1seed__subcmd__sign__subcmd__data_commands] )) ||
_1seed__subcmd__sign__subcmd__data_commands() {
    local commands; commands=()
    _describe -t commands '1seed sign data commands' commands "$@"
}
(( $+functions[_1seed__subcmd__sign__subcmd__help_commands] )) ||
_1seed__subcmd__sign__subcmd__help_commands() {
    local commands; commands=(
'pub:Show signing public key' \
'data:Sign data' \
'verify:Verify signature' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed sign help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__sign__subcmd__help__subcmd__data_commands] )) ||
_1seed__subcmd__sign__subcmd__help__subcmd__data_commands() {
    local commands; commands=()
    _describe -t commands '1seed sign help data commands' commands "$@"
}
(( $+functions[_1seed__subcmd__sign__subcmd__help__subcmd__help_commands] )) ||
_1seed__subcmd__sign__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands '1seed sign help help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__sign__subcmd__help__subcmd__pub_commands] )) ||
_1seed__subcmd__sign__subcmd__help__subcmd__pub_commands() {
    local commands; commands=()
    _describe -t commands '1seed sign help pub commands' commands "$@"
}
(( $+functions[_1seed__subcmd__sign__subcmd__help__subcmd__verify_commands] )) ||
_1seed__subcmd__sign__subcmd__help__subcmd__verify_commands() {
    local commands; commands=()
    _describe -t commands '1seed sign help verify commands' commands "$@"
}
(( $+functions[_1seed__subcmd__sign__subcmd__pub_commands] )) ||
_1seed__subcmd__sign__subcmd__pub_commands() {
    local commands; commands=()
    _describe -t commands '1seed sign pub commands' commands "$@"
}
(( $+functions[_1seed__subcmd__sign__subcmd__verify_commands] )) ||
_1seed__subcmd__sign__subcmd__verify_commands() {
    local commands; commands=()
    _describe -t commands '1seed sign verify commands' commands "$@"
}
(( $+functions[_1seed__subcmd__ssh_commands] )) ||
_1seed__subcmd__ssh_commands() {
    local commands; commands=(
'pub:Show SSH public key' \
'key:Show SSH private key' \
'add:Add SSH key to agent' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed ssh commands' commands "$@"
}
(( $+functions[_1seed__subcmd__ssh__subcmd__add_commands] )) ||
_1seed__subcmd__ssh__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands '1seed ssh add commands' commands "$@"
}
(( $+functions[_1seed__subcmd__ssh__subcmd__help_commands] )) ||
_1seed__subcmd__ssh__subcmd__help_commands() {
    local commands; commands=(
'pub:Show SSH public key' \
'key:Show SSH private key' \
'add:Add SSH key to agent' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands '1seed ssh help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__ssh__subcmd__help__subcmd__add_commands] )) ||
_1seed__subcmd__ssh__subcmd__help__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands '1seed ssh help add commands' commands "$@"
}
(( $+functions[_1seed__subcmd__ssh__subcmd__help__subcmd__help_commands] )) ||
_1seed__subcmd__ssh__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands '1seed ssh help help commands' commands "$@"
}
(( $+functions[_1seed__subcmd__ssh__subcmd__help__subcmd__key_commands] )) ||
_1seed__subcmd__ssh__subcmd__help__subcmd__key_commands() {
    local commands; commands=()
    _describe -t commands '1seed ssh help key commands' commands "$@"
}
(( $+functions[_1seed__subcmd__ssh__subcmd__help__subcmd__pub_commands] )) ||
_1seed__subcmd__ssh__subcmd__help__subcmd__pub_commands() {
    local commands; commands=()
    _describe -t commands '1seed ssh help pub commands' commands "$@"
}
(( $+functions[_1seed__subcmd__ssh__subcmd__key_commands] )) ||
_1seed__subcmd__ssh__subcmd__key_commands() {
    local commands; commands=()
    _describe -t commands '1seed ssh key commands' commands "$@"
}
(( $+functions[_1seed__subcmd__ssh__subcmd__pub_commands] )) ||
_1seed__subcmd__ssh__subcmd__pub_commands() {
    local commands; commands=()
    _describe -t commands '1seed ssh pub commands' commands "$@"
}
(( $+functions[_1seed__subcmd__status_commands] )) ||
_1seed__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands '1seed status commands' commands "$@"
}

if [ "$funcstack[1]" = "_1seed" ]; then
    _1seed "$@"
else
    compdef _1seed 1seed
fi
