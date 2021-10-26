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

#if !defined(OGS_APP_INSIDE) && !defined(OGS_APP_COMPILATION)
#error "This header cannot be included directly."
#endif

#ifndef OGS_APP_INIT_H
#define OGS_APP_INIT_H

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

int ogs_app_initialize(
        const char *version, const char *default_config,
        const char *const argv[]);
void ogs_app_terminate(void);

int ogs_app_config_read(void);
void ogs_app_setup_log(void);

#ifdef __cplusplus
}
#endif

#endif /* OGS_APP_INIT_H */
