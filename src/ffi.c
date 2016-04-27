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
