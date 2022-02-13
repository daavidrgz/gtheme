pcall(require, "luarocks.loader")

local gears = require("gears")
local awful = require("awful")
local wibox = require("wibox")
require("awful.hotkeys_popup.keys")

client.connect_signal("request::titlebars", function(c)
    -- buttons for the titlebar
    local buttons = gears.table.join(
        awful.button({ }, 1, function()
            c:emit_signal("request::activate", "titlebar", {raise = true})
            awful.mouse.client.move(c)
        end),
        awful.button({ }, 3, function()
            c:emit_signal("request::activate", "titlebar", {raise = true})
            awful.mouse.client.resize(c)
        end)
    )

    local close = awful.titlebar.widget.closebutton(c)
    local float = awful.titlebar.widget.floatingbutton(c)

    local titlebar = awful.titlebar(c, {
        size      = 30,
        position  = "top",
        bg_normal = '#FFF4DE',
        bg_focus  = '#FFF4DE',
        fg_normal = "#272727",
        fg_focus  = "#272727",
    })

    titlebar : setup {
        {
            wibox.layout.margin(close, 10, 0, 7, 5),
            wibox.layout.margin(float, 10, 0, 7, 5),
            layout = wibox.layout.fixed.horizontal,
            widget
        },
        {
            {
                align  = 'center',
                widget = awful.titlebar.widget.titlewidget(c)
            },
        buttons = buttons,
        layout  = wibox.layout.flex.horizontal
        },
        {  
            wibox.layout.margin(awful.titlebar.widget.iconwidget(c), 0, 25, 0, 0),
            buttons = buttons,
            layout = wibox.layout.fixed.horizontal,
            opacity = 0,
        },
        layout = wibox.layout.align.horizontal
    }
end)
