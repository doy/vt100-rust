#include <stdlib.h>
#include "../libvt100/src/vt100.h"

int vt100_wrapper_rows(VT100Screen *vt)
{
    return vt->grid->max.row;
}

int vt100_wrapper_cols(VT100Screen *vt)
{
    return vt->grid->max.col;
}

int vt100_wrapper_cell_is_wide(struct vt100_cell *cell)
{
    return cell->is_wide;
}

int vt100_wrapper_cell_bold(struct vt100_cell *cell)
{
    return cell->attrs.bold;
}

int vt100_wrapper_cell_italic(struct vt100_cell *cell)
{
    return cell->attrs.italic;
}

int vt100_wrapper_cell_underline(struct vt100_cell *cell)
{
    return cell->attrs.underline;
}

int vt100_wrapper_cell_inverse(struct vt100_cell *cell)
{
    return cell->attrs.inverse;
}
