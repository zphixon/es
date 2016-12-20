#!/usr/bin/env python
# encoding: utf8

APPNAME = "es"
VERSION = "0.1.0"

top = "."
out = "build"

def configure(ctx):
    print("project config in " + ctx.path.abspath())

def ping(ctx):
    print("ping from " + ctx.path.abspath())
    ctx.recurse('src')

