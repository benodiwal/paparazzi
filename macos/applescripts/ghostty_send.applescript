on run argv
    set messageText to item 1 of argv

    -- First, copy the message to clipboard using pbcopy
    set the clipboard to messageText

    tell application "System Events"
        tell process "Ghostty"
            set frontmost to true

            -- Look for windows
            set windowList to windows
            if (count of windowList) > 0 then
                -- Focus the first/current window (assuming it's the active one)
                set targetWindow to item 1 of windowList
                perform action "AXRaise" of targetWindow
                set focused of targetWindow to true

                -- Wait a moment for focus
                delay 0.3

                -- Paste the content using Cmd+V
                key code 9 using command down

                -- Press return
                delay 0.1
                key code 36
                return "true"
            end if
        end tell
    end tell
    return "false"
end run