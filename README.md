## Hyprland config

To make shell show blur and transparency correctly use
```
# Blur normal windows and Gtk.Popover popups
layerrule = blur, seashell
layerrule = blurpopups, seashell
# Don't blur shadows (below 0.24), blur window backgrounds
layerrule = ignorealpha 0.24, seashell
# Leave animations to app itself
layerrule = noanim, seashell
```
