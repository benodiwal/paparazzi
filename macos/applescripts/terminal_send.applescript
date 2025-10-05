on run argv
    set messageText to item 1 of argv

    tell application "Terminal"
        set foundTab to false

        -- Loop through all windows
        repeat with w in windows
            -- Loop through all tabs in the window
            repeat with t in tabs of w
                -- Get the tab's processes
                set tabProcesses to processes of t

                -- Check if any process contains "claude"
                repeat with p in tabProcesses
                    if p contains "claude" or p contains "Claude" then
                        -- Switch to this window and tab
                        set frontmost of w to true
                        set selected of t to true
                        activate

                        -- Send the text
                        do script messageText in t
                        set foundTab to true
                        exit repeat
                    end if
                end repeat
                if foundTab then exit repeat
            end repeat
            if foundTab then exit repeat
        end repeat

        return foundTab
    end tell
end run