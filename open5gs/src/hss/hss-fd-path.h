/*
 * Copyright (C) 2019 by Sukchan Lee <acetcom@gmail.com>
 *
 * This file is part of Open5GS.
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

#ifndef HSS_FD_PATH_H
#define HSS_FD_PATH_H

#ifdef __cplusplus
extern "C" {
#endif

int hss_fd_init(void);
void hss_fd_final(void);

int hss_s6a_init(void);
void hss_s6a_final(void);
int hss_cx_init(void);
void hss_cx_final(void);
int hss_swx_init(void);
void hss_swx_final(void);

#ifdef __cplusplus
}
#endif

#endif /* HSS_FD_PATH_H */

