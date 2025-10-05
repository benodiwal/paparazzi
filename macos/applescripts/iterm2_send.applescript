on run argv
    set messageText to item 1 of argv

    tell application "iTerm"
        set foundSession to false

        -- Loop through all windows
        repeat with w in windows
            -- Loop through all tabs in the window
            repeat with t in tabs of w
                -- Loop through all sessions in the tab
                repeat with s in sessions of t
                    -- Get the session content
                    set sessionText to contents of s

                    -- Check if this session contains Claude Code indicators
                    if sessionText contains "claude" or sessionText contains "Claude" then
                        -- Switch to this window
                        select w
                        -- Switch to this tab
                        select t
                        -- Switch to this session
                        select s
                        -- Send the text
                        tell s to write text messageText
                        set foundSession to true
                        exit repeat
                    end if
                end repeat
                if foundSession then exit repeat
            end repeat
            if foundSession then exit repeat
        end repeat

        if foundSession then
            activate
            return "true"
        else
            return "false"
        end if
    end tell
end run