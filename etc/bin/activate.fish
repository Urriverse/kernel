# only copy if fish_prompt exists
if functions -q fish_prompt
    functions -c fish_prompt _old_fish_prompt 2>/dev/null
end

function fish_prompt
    printf 'kernel '
    if functions -q _old_fish_prompt
        _old_fish_prompt
    end
end

function fish_greeting
    do -h
end
