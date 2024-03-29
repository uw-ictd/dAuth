# Copyright (C) 2019,2020 by Sukchan Lee <acetcom@gmail.com>

# This file is part of Open5GS.

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

libausf_sources = files('''
    context.c
    event.c
    timer.c

    nnrf-build.c
    nnrf-handler.c
    nf-sm.c

    nausf-handler.c

    nudm-build.c
    nudm-handler.c
    ue-sm.c

    dauth-c-binding.cpp
    dauth-server-context.cpp
    dauth-local-auth-client.cpp

    sbi-path.c
    ausf-sm.c

    init.c
'''.split())

libausf_sources += grpc_sources

libausf_deps = [
    libapp_dep,
    libcrypt_dep,
    libsbi_dep]

libausf_deps += grpc_deps

libausf = static_library('ausf',
    sources : libausf_sources,
    dependencies : libausf_deps,
    install : false)

libausf_dep = declare_dependency(
    link_with : libausf,
    dependencies : libausf_deps)

ausf_sources = files('''
    app.c
    ../main.c
'''.split())

executable('open5gs-ausfd',
    sources : ausf_sources,
    c_args : '-DDEFAULT_CONFIG_FILENAME="@0@/ausf.yaml"'.format(open5gs_sysconfdir),
    include_directories : srcinc,
    dependencies : [libausf_dep],
    install_rpath : libdir,
    install : true)
