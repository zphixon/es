
#include <stdio.h>
#include <ncurses.h>
#include "lua.h"
#include "lauxlib.h"

int main(int argc, char **argv) {
    // lua test
    lua_State *L = luaL_newstate();
    luaL_openlibs(L);
    luaL_dofile(L, "test.lua");

    // ncurses test
    initscr();
    cbreak();
    printw("Hello from ncurses");
    getch();
    endwin();

    return 0;
}

