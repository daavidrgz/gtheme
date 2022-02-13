local awful = require("awful")
local gears = require("gears")
local wibox = require("wibox")
local beautiful = require("beautiful")
local dpi = beautiful.xresources.apply_dpi

-- Set colors
local active_color = {
    color = "#CEF6DF"
}

local background_color = "#ffffff"

local cpu_bar = wibox.widget {
    max_value = 100,
    value = 50,
    forced_height = 30,
    forced_width = 350,
    shape = function(cr,w,h) gears.shape.rounded_rect(cr,w,h, 8) end,
    bar_shape = function(cr,w,h) gears.shape.rounded_rect(cr,w,h, 4) end,
    color = active_color,
    background_color = background_color,
    border_width = 5,
    border_color = "#272727",
    widget = wibox.widget.progressbar
}

awesome.connect_signal("signals::cpu", function(value)
    cpu_bar.value = value
end)

return cpu_bar