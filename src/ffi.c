#include <stdlib.h>
#include "../libvt100/src/vt100.h"

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
