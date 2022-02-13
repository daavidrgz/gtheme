local gears = require("gears")
local theme_assets = require("beautiful.theme_assets")
local xresources = require("beautiful.xresources")
local dpi = xresources.apply_dpi
local xrdb = xresources.get_current_theme()
local gfs = require('gears.filesystem')
local themes_path = gfs.get_themes_dir()
local beautiful = require("beautiful")

local theme = {}

-- themes

theme.font          = "RobotoMono Nerd Font Bold 12"

theme.dir = string.format('%s/.config/awesome/theme', os.getenv('HOME'))

theme.bg_normal     = "#FFF4DE"
theme.bg_focus      = "#272727"

theme.fg_normal     = "#272727"
theme.fg_focus     = "#FFF4DE"

theme.useless_gap   = 12
theme.border_width  = 4
theme.border_normal = "#272727"
theme.border_focus  = "#272727"
theme.border_marked = "#272727"

-- widgets

theme.taglist_fg = "#272727"


-- menu

theme.menu_submenu_icon = gfs.get_configuration_dir() .. "theme/submenu.png"
theme.menu_height = dpi(25)
theme.menu_width = dpi(120)
theme.menu_bg = "#FFF4DE"
theme.menu_font = "RobotoMono Nerd Font Medium 12"

-- awesome icon

theme.awesome_icon = gfs.get_configuration_dir() .. "theme/awesome.png"

-- tag preview

theme.tag_preview_widget_border_radius = 16        
theme.tag_preview_client_border_radius = 4
theme.tag_preview_client_opacity = 1
theme.tag_preview_client_bg = theme.bg_normal        
theme.tag_preview_client_border_color = "#272727"  
theme.tag_preview_client_border_width = 2   
theme.tag_preview_widget_bg = theme.bg_normal          
theme.tag_preview_widget_border_color = "#272727"   
theme.tag_preview_widget_border_width = 4    
theme.tag_preview_widget_margin = 5          

-- titlebar

theme.titlebar_close_button_normal = gfs.get_configuration_dir() .. "theme/titlebar/circle.png"
theme.titlebar_close_button_focus  = gfs.get_configuration_dir() .. "theme/titlebar/circle.png"
theme.titlebar_close_button_focus_hover = gfs.get_configuration_dir() .. "theme/titlebar/close_hover.png"

theme.titlebar_floating_button_normal_inactive = gfs.get_configuration_dir() .. "theme/titlebar/circle.png"
theme.titlebar_floating_button_focus_inactive  = gfs.get_configuration_dir() .. "theme/titlebar/circle.png"
theme.titlebar_floating_button_normal_active = gfs.get_configuration_dir() .. "theme/titlebar/circle.png"
theme.titlebar_floating_button_focus_active  = gfs.get_configuration_dir() .. "theme/titlebar/circle.png"
theme.titlebar_floating_button_focus_active_hover  = gfs.get_configuration_dir() .. "theme/titlebar/floating_hover.png"
theme.titlebar_floating_button_focus_inactive_hover  = gfs.get_configuration_dir() .. "theme/titlebar/floating_hover.png"

theme.icon_theme = nil

return theme
