# ese editor

**e**asily **s**criptable **e**ditor

Pronounced *easy*

* ketos: lisp dialect for scripting
* pancurses: curses for rust

## Architecture

* Minimal rust library to handle text buffers and keyboard events
* Ketos library to interface with events and text manipulation
* User plugins to interface with ketos library and events

```scheme
(register-callback "startup"
                   (lambda ()
                     (display-message "Hello!")))
```

