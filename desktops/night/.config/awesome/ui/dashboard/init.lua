-- Standard awesome library
local gears = require("gears")
local awful = require("awful")

-- Theme handling library
local beautiful = require("beautiful")
local dpi = beautiful.xresources.apply_dpi

-- Widget library
local wibox = require("wibox")

-- rubato
local rubato = require("module.rubato")

-- Helpers
local helpers = require("helpers")

-- Get screen geometry
local screen_width = awful.screen.focused().geometry.width
local screen_height = awful.screen.focused().geometry.height


-- dashboard
------------

-- Helpers
local function centered_widget(widget)
    local w = wibox.widget{
        nil,
        {
            nil,
            widget,
            expand = "none",
            layout = wibox.layout.align.vertical
        },
        expand = "none",
        layout = wibox.layout.align.horizontal
    }

    return w
end

local function create_boxed_widget(widget_to_be_boxed, width, height, bg_color)
    local box_container = wibox.container.background()
    box_container.bg = bg_color
    box_container.forced_height = height
    box_container.forced_width = width
    box_container.shape = helpers.rrect(dpi(5))

    local boxed_widget = wibox.widget {
        -- Add margins
        {
            -- Add background color
            {
                -- The actual widget goes here
                widget_to_be_boxed,
                top = dpi(9),
                bottom = dpi(9),
                left = dpi(10),
                right = dpi(10),
                widget = wibox.container.margin
            },
            widget = box_container,
        },
        margins = dpi(10),
        color = "#FF000000",
        widget = wibox.container.margin
    }

    return boxed_widget
end


-- Widget
local profile = require("ui.dashboard.profile")
local music = require("ui.dashboard.music")
local media = require("ui.dashboard.mediakeys")
local time = require("ui.dashboard.time")
local date = require("ui.dashboard.date")
local todo = require("ui.dashboard.todo")
local weather = require("ui.dashboard.weather")
local stats = require("ui.dashboard.stats")
local notifs = require("ui.dashboard.notifs")


local time_boxed = create_boxed_widget(centered_widget(time), dpi(260), dpi(95), beautiful.transparent)
local date_boxed = create_boxed_widget(date, dpi(120), dpi(50), beautiful.dashboard_box_bg)
local todo_boxed = create_boxed_widget(todo, dpi(120), dpi(120), beautiful.dashboard_box_bg)
local weather_boxed = create_boxed_widget(weather, dpi(120), dpi(120), beautiful.dashboard_box_bg)
local stats_boxed = create_boxed_widget(stats, dpi(120), dpi(190), beautiful.dashboard_box_bg)
local notifs_boxed = create_boxed_widget(notifs, dpi(260), dpi(190), beautiful.dashboard_box_bg)

-- dashboard
dashboard = wibox({
    type = "dock",
    screen = screen.primary,
    height = screen_height - dpi(50),
    width = beautiful.dashboard_width or dpi(300),
    shape = helpers.rrect(beautiful.border_radius),
    ontop = true,
    visible = false
})
dashboard.y = dpi(25)


local slide = rubato.timed{
    pos = dpi(-275),
    rate = 120,
    intro = 0.05,
    duration = 0.25,
    easing = rubato.linear,
    awestore_compat = true,
    subscribed = function(pos) dashboard.x = pos end
}

local dashboard_status = false

slide.ended:subscribe(function()
    if dashboard_status then
        dashboard.visible = false
    end
end)

dashboard_show = function()
    dashboard.visible = true
    slide:set(dpi(100))
    dashboard_status = false
end

dashboard_hide = function()
    slide:set(dpi(-275))
    dashboard_status = true
end

dashboard_toggle = function()
    if dashboard.visible then
        dashboard_hide()
    else
        dashboard_show()
    end
end

dashboard:setup {
    {
        {
            nil,
            {
                {
                    {
                        profile,
                        stats_boxed,
                        layout = wibox.layout.fixed.vertical
                    },
                    {
                        date_boxed,
                        todo_boxed,
                        weather_boxed,
                        layout = wibox.layout.fixed.vertical
                    },
                    layout = wibox.layout.fixed.horizontal
                },
                {
                    music,
                    media,
                    layout = wibox.layout.fixed.horizontal
                },
                notifs_boxed,
                layout = wibox.layout.fixed.vertical
                },
                expand = "none",
                layout = wibox.layout.align.horizontal
            },
            margins = dpi(10),
            widget = wibox.container.margin
        },
        bg = beautiful.xbackground,
        shape = helpers.rrect(beautiful.dashboard_radius),
        widget = wibox.container.background
    }
