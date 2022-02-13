local dpi = require('beautiful.xresources').apply_dpi
local wibox  = require('wibox')
local awful = require('awful')
local icons = require('config.icons')
local gears = require("gears")

desktop = "kitty ranger"
trashbin = 'firefox "https://www.youtube.com/watch?v=dQw4w9WgXcQ"'
internet = browser
download = "kitty --working-directory /home/david/Downloads"

awful.screen.connect_for_each_screen(function (scr)

desktop_icons = wibox({visible = true, ontop = false, type = "dock", screen = screen.primary})
desktop_icons.bg = "#00000000"
desktop_icons.fg = "#272727"
desktop_icons.height = 900
desktop_icons.width = 100
desktop_icons.y = 120
desktop_icons.x = 1780

local desktop_textbox = wibox.widget {
      markup = "Desktop",
      align  = 'center',
      valign = 'center',
      widget = wibox.widget.textbox,
      font = "RobotoMono Nerd Font Bold 11"
  }

local download_textbox = wibox.widget {
   markup = "Downloads",
   align  = 'center',
   valign = 'center',
   font = "RobotoMono Nerd Font Bold 11",
   widget = wibox.widget.textbox
}

local internet_textbox = wibox.widget {
    markup = "Internet",
    align  = 'center',
    valign = 'center',
    widget = wibox.widget.textbox,
    font = "RobotoMono Nerd Font Bold 11",
 }

local trashbin_textbox = wibox.widget {

    markup = "My Life",
    align  = 'center',
    valign = 'center',
    widget = wibox.widget.textbox,
    font = "RobotoMono Nerd Font Bold 11",
   }

   local function create_img_widget(image, apps)
      local widget = wibox.widget {
         image = image,
         forced_height = 60,
         resize = true,
         widget = wibox.widget.imagebox()
      }
      widget:buttons(gears.table.join(awful.button({}, 1, function()
         awful.spawn(apps)
     end)))
      return widget
   end

   local desktop = create_img_widget(icons.png.desktop, desktop)
   local download = create_img_widget(icons.png.folder, download)
   local internet = create_img_widget(icons.png.internet, internet)
   local trashbin = create_img_widget(icons.png.trashbin, trashbin)

   desktop_icons : setup {
      {
         wibox.layout.margin(desktop, 18, 0, 24, 0), 
         desktop_textbox,
         wibox.layout.margin(download, 13, 0, 24, 0), 
         download_textbox,
         wibox.layout.margin(internet, 23, 0, 24, 0), 
         internet_textbox,
         spacing = 10,
         layout = wibox.layout.fixed.vertical,
      },
      {        
         layout = wibox.layout.fixed.vertical,
      },
      {
         wibox.layout.margin(trashbin, 23, 0, 24, 0), 
         trashbin_textbox,
         spacing = 10,
         layout = wibox.layout.fixed.vertical,
      },
      layout = wibox.layout.align.vertical
   }
end)
