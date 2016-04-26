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
