# Hook: Auto-detect and show info
opn_chpwd() {
    if [[ -f ".opn" ]]; then
        
    elif [[ -f "$HOME/.opn" ]] && [[ ! -f ".opn" ]]; then
       
    fi
}