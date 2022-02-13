local gears = require("gears")
local awful = require("awful")
local beautiful = require("beautiful")
local gfs = require("gears.filesystem")
require("awful.autofocus")

beautiful.init(gfs.get_configuration_dir() .. "theme/theme.lua")

terminal = "kitty"
browser = "firefox"
fm = "ranger"
vscode = "code"
discord = "discord"
editor = os.getenv("EDITOR") or "nvim"
editor_cmd = terminal .. " -e " .. editor

myawesomemenu = {
    { "Restart", awesome.restart },
    { "Quit", function() awesome.quit() end },
 }
  
myapps = {
    { "Browser", browser },
    { "Editor", vscode },
    { "Terminal", terminal },
    { "Discord", discord }
}

mymainmenu = awful.menu(
{ items = { 
    { "awesome", myawesomemenu, beautiful.awesome_icon},
    { "apps", myapps},
  }
})
