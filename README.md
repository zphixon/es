# es editor

**e**asily **s**criptable

* ketos: lisp dialect for scripting
* pancurses: curses for rust

## Architecture

* Minimal rust library to handle text buffers and keyboard events
* Ketos library to interface with events and text manipulation
* User plugins to interface with ketos library and events

```scheme
; this is currently in the works

; main startup callback in user config
(define (es-user-main global major minor current-buffer editor)
  (do
    ; add new modes
    (new-mode global "vim-lite" "add vim-style keybinds")
    (new-mode major "vim-lite" "vim-lite major")
    (new-mode minor "vim-lite" "vim-lite minor")

    ; start in normal mode by default
    (add-attr current-buffer ("insert" false))

    ; movement: global is entire es session
    (add-key global "vim-lite" "j" (lambda (ctx)
                                     (if (attr current-buffer "insert")
                                       (put-char current-buffer (. ctx :key))
                                       (down-line 1))))
    (add-key global "vim-lite" "k" (lambda (ctx)
                                     (if (attr current-buffer "insert")
                                       (put-char current-buffer (. ctx :key))
                                       (up-line 1))))
    (add-key global "vim-lite" "h" (lambda (ctx)
                                     (if (attr current-buffer "insert")
                                       (put-char current-buffer (. ctx :key))
                                       (left-char 1))))
    (add-key global "vim-lite" "l" (lambda (ctx)
                                     (if (attr current-buffer "insert")
                                       (put-char current-buffer (. ctx :key))
                                       (right-char 1))))

    ; insert/normal modes: buffer-specific is minor
    (add-key minor "vim-lite" "i" (lambda (ctx)
                                     (if (= true (attr current-buffer "insert"))
                                       (put-char (. ctx :key))
                                       (set-attr current-buffer "insert" true))))
    (add-key minor "vim-lite" "esc" (lambda (ctx)
                                       (if (= true (attr current-buffer "insert"))
                                         (set-attr current-buffer "insert" false))))

    ; set modes to vim-lite
    (set-mode global "vim-lite")
    (set-mode major "vim-lite")
    (set-mode minor "vim-lite")))
```

