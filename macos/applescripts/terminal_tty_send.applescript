on run argv
    set ttyPath to item 1 of argv
    set messageText to item 2 of argv

    tell application "Terminal"
        set foundTab to false

        repeat with w in windows
            repeat with t in tabs of w
                set currentTTY to tty of t

                if currentTTY is equal to ttyPath then
                    set selected of t to true
                    set frontmost of w to true
                    activate

                    do script messageText in t
                    set foundTab to true
                    exit repeat
                end if
            end repeat

            if foundTab then exit repeat
        end repeat

        return foundTab
    end tell
end run