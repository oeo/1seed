#compdef 1seed

_1seed() {
    local -a commands
    commands=(
        'pub:Age public key'
        'priv:Age private key'
        'ssh:SSH private key'
        'ssh-pub:SSH public key'
        'sign-pub:Ed25519 signing public key'
        'enc:Encrypt with age'
        'dec:Decrypt with age'
        'sign:Sign data'
        'verify:Verify signature'
        'pw:Derive password'
        'raw:Derive raw bytes'
        'mnemonic:Derive BIP39 mnemonic'
        'ssh-add:Add SSH key to agent'
        'config:Manage configuration'
        'realms:Manage known realms'
        'info:Show status'
    )

    local -a global_opts
    global_opts=(
        '(-r --realm)'{-r,--realm}'[Realm]:realm:->realms'
        '(-f --seed-file)'{-f,--seed-file}'[Seed file]:file:_files'
        '(-h --help)'{-h,--help}'[Show help]'
        '(-V --version)'{-V,--version}'[Show version]'
    )

    local state

    _arguments -C \
        $global_opts \
        '1: :->command' \
        '*:: :->args'

    case $state in
        command)
            _describe -t commands 'command' commands
            ;;
        realms)
            local -a known_realms
            known_realms=(${(f)"$(1seed realms 2>/dev/null)"})
            _describe -t realms 'realm' known_realms
            ;;
        args)
            case $words[1] in
                enc)
                    _arguments \
                        '*'{-R,--recipient}'[Recipient]:recipient:' \
                        '*'{-F,--recipients-file}'[Recipients file]:file:_files' \
                        '(-s --self)'{-s,--self}'[Include self]' \
                        '(-p --passphrase)'{-p,--passphrase}'[Use passphrase]' \
                        '(-a --armor)'{-a,--armor}'[ASCII armor]' \
                        '(-o --output)'{-o,--output}'[Output file]:file:_files' \
                        '*:file:_files'
                    ;;
                dec)
                    _arguments \
                        '(-k --key)'{-k,--key}'[Key file]:file:_files' \
                        '(-p --passphrase)'{-p,--passphrase}'[Use passphrase]' \
                        '(-o --output)'{-o,--output}'[Output file]:file:_files' \
                        '*:file:_files'
                    ;;
                sign)
                    _arguments \
                        '(-o --output)'{-o,--output}'[Output file]:file:_files' \
                        '--binary[Binary output]' \
                        '*:file:_files'
                    ;;
                verify)
                    _arguments \
                        '(-k --pubkey)'{-k,--pubkey}'[Public key]:key:' \
                        '1:signature:' \
                        '*:file:_files'
                    ;;
                pw)
                    _arguments \
                        '(-l --length)'{-l,--length}'[Length]:length:' \
                        '(-n --counter)'{-n,--counter}'[Counter]:counter:' \
                        '--no-symbols[No symbols]' \
                        '--symbols[Symbol set]:symbols:' \
                        '1:site:'
                    ;;
                raw)
                    _arguments \
                        '(-l --length)'{-l,--length}'[Length]:length:' \
                        '--hex[Hex output]' \
                        '--base64[Base64 output]' \
                        '--binary[Binary output]' \
                        '1:path:'
                    ;;
                mnemonic)
                    _arguments \
                        '(-w --words)'{-w,--words}'[Word count]:words:(12 15 18 21 24)'
                    ;;
                ssh-add)
                    _arguments \
                        '(-t --lifetime)'{-t,--lifetime}'[Lifetime]:seconds:' \
                        '(-c --confirm)'{-c,--confirm}'[Require confirmation]'
                    ;;
                config)
                    local -a config_cmds
                    config_cmds=(
                        'set:Set value'
                        'get:Get value'
                        'path:Show path'
                    )
                    _describe -t commands 'config command' config_cmds
                    ;;
                realms)
                    local -a realms_cmds
                    realms_cmds=(
                        'add:Add realm'
                        'rm:Remove realm'
                    )
                    _describe -t commands 'realms command' realms_cmds
                    ;;
            esac
            ;;
    esac
}

_1seed
