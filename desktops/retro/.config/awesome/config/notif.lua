local awful = require("awful")
local gears = require("gears")
local wibox = require("wibox")
local beautiful = require("beautiful")
local naughty = require("naughty")
local gfs = require("gears.filesystem")

naughty.config.defaults.ontop = true
naughty.config.defaults.icon_size = 70
naughty.config.defaults.screen = awful.screen.focused()
naughty.config.defaults.timeout = 5
naughty.config.defaults.title = "System Notification"
naughty.config.defaults.margin = 20
naughty.config.defaults.position = "bottom_left"
naughty.config.defaults.border_width = 3
naughty.config.defaults.border_color = "#272727"
naughty.config.defaults.font = "RobotoMono Nerd Font Medium 11"
naughty.config.defaults.width = 300
