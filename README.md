# es editor

es stands for
* **e**asily **s**criptable
* **e**vent **s**tuff

## TODO

Everything.

## Roadmap

1. Set up a barebones environment
    * ncurses/something?
    * es struct
        * Global mode
        * Windows
            * Macro mode
            * Buffers
                * Micro mode
                * Cursor(s?)
                * File
                    * Real file or in memory
                    * Saved?
            * Position
        * Keystroke buffer
2. Modal as in emacs *and* vim
    * Global/macro/micro modes
        * Global is editing style, vim/nano/emacs
            * sent all keystrokes
            * non-intercepted keystrokes go straight to file
        * Macro is typical plugins, netrw, org-mode style stuff
            * sent only some of the keystrokes
        * Micro is syntax highlighting, indentation style
            * very few strokes sent, sometimes none
    * Keystroke passage order
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
3. Functions that mutate states
    * Cursor movement
    * Insertion/deletion of characters
    * Movement of characters
    * Save/open/close files
    * Open/close/hide/show buffers
    * Open/close/move windows
    * Close editor
    * Change global/macro/micro modes
4. Scripting - where this all is implemented
    * Lua
    * Scheme
    * Tcl
5. Help/debugging system
    * Log keystroke passage: toggled to protect privacy, not remembered
    * Show editor state
    * Show registered mappings
        * Registered list is not required but strongly reccommended

