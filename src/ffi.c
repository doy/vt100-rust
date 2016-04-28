#include <stdlib.h>
#include "../libvt100/src/vt100.h"

int vt100_wrapper_screen_hide_cursor(struct vt100_screen *screen)
{
    return screen->hide_cursor;
}

int vt100_wrapper_screen_application_keypad(struct vt100_screen *screen)
{
    return screen->application_keypad;
}

int vt100_wrapper_screen_application_cursor(struct vt100_screen *screen)
{
    return screen->application_cursor;
}

int vt100_wrapper_screen_mouse_reporting_press(struct vt100_screen *screen)
{
    return screen->mouse_reporting_press;
}

int vt100_wrapper_screen_mouse_reporting_press_release(struct vt100_screen *screen)
{
    return screen->mouse_reporting_press_release;
}

int vt100_wrapper_screen_mouse_reporting_button_motion(struct vt100_screen *screen)
{
    return screen->mouse_reporting_button_motion;
}

int vt100_wrapper_screen_mouse_reporting_sgr_mode(struct vt100_screen *screen)
{
    return screen->mouse_reporting_sgr_mode;
}

int vt100_wrapper_screen_bracketed_paste(struct vt100_screen *screen)
{
    return screen->bracketed_paste;
}

int vt100_wrapper_screen_visual_bell(struct vt100_screen *screen)
{
    return screen->visual_bell;
}

int vt100_wrapper_screen_audible_bell(struct vt100_screen *screen)
{
    return screen->audible_bell;
}

int vt100_wrapper_screen_update_title(struct vt100_screen *screen)
{
    return screen->update_title;
}

int vt100_wrapper_screen_update_icon_name(struct vt100_screen *screen)
{
    return screen->update_icon_name;
}

int vt100_wrapper_screen_dirty(struct vt100_screen *screen)
{
    return screen->dirty;
}

void vt100_wrapper_screen_clear_visual_bell(struct vt100_screen *screen)
{
    screen->visual_bell = 0;
}

void vt100_wrapper_screen_clear_audible_bell(struct vt100_screen *screen)
{
    screen->audible_bell = 0;
}

void vt100_wrapper_screen_clear_update_title(struct vt100_screen *screen)
{
    screen->update_title = 0;
}

void vt100_wrapper_screen_clear_update_icon_name(struct vt100_screen *screen)
{
    screen->update_icon_name = 0;
}

void vt100_wrapper_screen_clear_dirty(struct vt100_screen *screen)
{
    screen->dirty = 0;
}

int vt100_wrapper_cell_is_wide(struct vt100_cell *cell)
{
    return cell->is_wide;
}

int vt100_wrapper_cell_attrs_bold(struct vt100_cell_attrs *attrs)
{
    return attrs->bold;
}

int vt100_wrapper_cell_attrs_italic(struct vt100_cell_attrs *attrs)
{
    return attrs->italic;
}

int vt100_wrapper_cell_attrs_underline(struct vt100_cell_attrs *attrs)
{
    return attrs->underline;
}

int vt100_wrapper_cell_attrs_inverse(struct vt100_cell_attrs *attrs)
{
    return attrs->inverse;
}
