#ifndef __MAGIC_H__
#define __MAGIC_H__ "magic.h"
 
#include <iostream>
#include <functional>
#include <algorithm>
 
#ifdef __cplusplus
// This extern block is used to avoid name mangling by cpp
extern "C" {
#endif
  size_t add(size_t, size_t);
#ifdef __cplusplus
}

#endif
#endif