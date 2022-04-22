/*
 * Copyright (C) 2021 Matt Johnson <matt9j@cs.washington.edu>
 *
 * This file is part of dAuth, and extends open5gs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef AUSF_DAUTH_CONTEXT_C_BINDING_H
#define AUSF_DAUTH_CONTEXT_C_BINDING_H

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct dauth_context_wrapper {
    void* dauth_context_internal;
} dauth_context_t;

bool
dauth_context_init(dauth_context_t * context);

bool
dauth_context_final(dauth_context_t * const context);

#ifdef __cplusplus
}
#endif

#endif /* AUSF_DAUTH_CONTEXT_C_BINDING_H */
