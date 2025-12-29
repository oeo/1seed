_1seed() {
    local cur prev words cword
    _init_completion || return

    local commands="pub priv ssh ssh-pub sign-pub enc dec sign verify pw raw mnemonic ssh-add config realms info"
    local config_actions="set get path"
    local realms_actions="add rm"

    case "${prev}" in
        1seed)
            COMPREPLY=($(compgen -W "${commands} -r --realm -f --seed-file -h --help -V --version" -- "${cur}"))
            return
            ;;
        -r|--realm)
            # Complete with known realms from config
            local realms=$(1seed realms 2>/dev/null)
            COMPREPLY=($(compgen -W "${realms}" -- "${cur}"))
            return
            ;;
        -f|--seed-file|-o|--output|-k|--key|-F|--recipients-file)
            _filedir
            return
            ;;
        config)
            COMPREPLY=($(compgen -W "${config_actions}" -- "${cur}"))
            return
            ;;
        realms)
            COMPREPLY=($(compgen -W "${realms_actions}" -- "${cur}"))
            return
            ;;
        set)
            COMPREPLY=($(compgen -W "realm seed-file" -- "${cur}"))
            return
            ;;
        get)
            COMPREPLY=($(compgen -W "realm seed-file" -- "${cur}"))
            return
            ;;
        add|rm)
            local realms=$(1seed realms 2>/dev/null)
            COMPREPLY=($(compgen -W "${realms}" -- "${cur}"))
            return
            ;;
        enc)
            COMPREPLY=($(compgen -W "-R --recipient -F --recipients-file -s --self -p --passphrase -a --armor -o --output" -- "${cur}"))
            return
            ;;
        dec)
            COMPREPLY=($(compgen -W "-k --key -p --passphrase -o --output" -- "${cur}"))
            return
            ;;
        sign)
            COMPREPLY=($(compgen -W "-o --output --binary" -- "${cur}"))
            return
            ;;
        verify)
            COMPREPLY=($(compgen -W "-k --pubkey" -- "${cur}"))
            return
            ;;
        pw)
            COMPREPLY=($(compgen -W "-l --length -n --counter --no-symbols --symbols" -- "${cur}"))
            return
            ;;
        raw)
            COMPREPLY=($(compgen -W "-l --length --hex --base64 --binary" -- "${cur}"))
            return
            ;;
        mnemonic)
            COMPREPLY=($(compgen -W "-w --words" -- "${cur}"))
            return
            ;;
        ssh-add)
            COMPREPLY=($(compgen -W "-t --lifetime -c --confirm" -- "${cur}"))
            return
            ;;
    esac

    COMPREPLY=($(compgen -W "${commands}" -- "${cur}"))
}

complete -F _1seed 1seed
