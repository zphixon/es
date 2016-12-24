# es editor [![Freaking circle](https://circleci.com/gh/cheezgi/es.svg?style=svg)](https://circleci.com/gh/cheezgi/es)

es stands for
* **e**asily **s**criptable
* **e**vent **s**tuff
* [crash**es** valgrind sometim**es**](http://i.imgur.com/rt0c7Ow.png)

## TODO

* Implement basic editing functions: see #3 below
* Expose those functions to Lua API
* Create default Lua mode stuff

## Roadmap

1. Set up a barebones environment
    * ncurses/something?
    * es struct
        * Global mode
        * Macro mode
        * Micro mode
        * Windows
            * Buffers
                * File
                    * Real file or in memory
                    * Saved?
            * Position
        * Keystroke buffer
        * Options - maybe not needed due to modes?
        * Status line for mode types?
        * Cursor
    * Some sort of global config file that load modes and stuff
2. Modal as in emacs *and* vim
    * Global/macro/micro modes
        * Global is editing style, vim/nano/emacs
            * sent all keystrokes
            * non-intercepted keystrokes go straight to file
        * Macro is typical plugins, netrw, org-mode style stuff
            * sent only some of the keystrokes
        * Micro is syntax highlighting, indentation style
            * very few strokes sent, sometimes none
        * All have access to file/buffer/window for reading/writing
    * Keystroke passage order
        * Register mapped keystrokes to pass to modes down the line
        * Global mode gets them first
        * Macro mode gets them second
        * Micro mode
        * File
    * Start with nano-style global mode
        * arrow keys do movement
        * control keys do commands
    * Later add vim-style global mode
        * vim movement
        * `:` commands mapped to es event triggers?
3. Functions that mutate states - exposed to Lua API
    * Cursor movement
    * Insertion/deletion of characters
    * Movement of characters
    * Save/open/close files
    * Open/close/hide/show buffers
    * Open/close/move windows
    * Close editor
    * Change global/macro/micro modes
    * Change file/buffer/window metadata used for modes
    * Commands for interoperability between modes
        * Some sort of binding to enter a command
    * Clipboard operation
4. Scripting - where this all is implemented
    * Modes are sent keystrokes/events
5. Help/debugging system
    * Log keystroke passage: toggled to protect privacy, not remembered
    * Show editor state
    * Show registered mappings
        * Registered list required
        * Modes can add maps up the chain

## Example global mode: Vim-lite

Keymaps:
* `h`: cursor left
* `j`: cursor down
* `k`: cursor up
* `l`: cursor right
* `i`: enter insert
* `Esc`: leave insert

How this might translate to Lua API

```lua
-- somewhere in esrc.lua
require("modes/vim-lite")

-- in vim-lite

-- keep track of whether we're in insert mode
es.currentWindow.insert = false

-- register a global mode
es.registerGlobal("vim-lite")

-- es.map takes several parameters:
-- the mode name: "vim-lite"
-- the key to map: "esc"
-- the callback for the key event
-- an optional docstring
es.map("vim-lite", "esc", function()
    if es.currentWindow.insert then
        insert = false
        es.setStatusline("NORMAL")
    end
end, "Leave insert mode")

es.map("vim-lite", "i", function()
    if not es.currentWindow.insert then
        es.currentWindow.insert = true
        es.statusline("INSERT")
    else
        -- send the keystroke down the line
        es.send()
    end
end)

es.map("vim-lite", "h", function()
    if not es.currentWindow.insert then
        es.cursor.left(1)
    else
        es.send()
    end
end, "Move left")

es.map("vim-lite", "j", function()
    if not es.currentWindow.insert then
        es.cursor.down(1)
    else
        es.send()
    end
end, "Move down")

es.map("vim-lite", "k", function()
    if not es.currentWindow.insert then
        es.cursor.up(1)
    else
        es.send()
    end
end, "Move up")

es.map("vim-lite", "l", function()
    if not es.currentWindow.insert then
        es.cursor.right(1)
    else
        es.send()
    end
end, "Move right")

-- send all unmapped keys down to the next layer - may be implicit later on?
es.map("vim-lite", es.unmappedKeys, function()
    if es.currentWidnow.insert then
        es.send()
    end
end)
```

