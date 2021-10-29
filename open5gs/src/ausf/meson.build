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

# Import the grpc subproject from the external cmake build system
cmake = import('cmake')
grpc_options = cmake.subproject_options()
grpc_options.set_install(false)
grpc_options.add_cmake_defines({'gRPC_INSTALL': 'OFF', 'gRPC_BUILD_TESTS': 'OFF'})

grpc_proj = cmake.subproject('grpc', options: grpc_options)
message('CMaket targets:\n - ' + '\n - '.join(grpc_proj.target_list()))
grpc_dep = grpc_proj.dependency('grpc++')
proto_dep = grpc_proj.dependency('libprotobuf')
grpc_cpp_plugin_dep = grpc_proj.target('grpc_cpp_plugin')

protoc_executable = grpc_proj.target('protoc')


gen = generator(protoc_executable,
  output    : ['@BASENAME@.pb.cc', '@BASENAME@.pb.h', '@BASENAME@.grpc.pb.cc', '@BASENAME@.grpc.pb.h'],
  depends: grpc_cpp_plugin_dep,
  arguments : [
    '--proto_path=../../protos',
    '--cpp_out=@BUILD_DIR@',
    '--grpc_out=@BUILD_DIR@',
    '--plugin=protoc-gen-grpc=subprojects/grpc/grpc_cpp_plugin',
    '@PLAINNAME@'])

generated = gen.process(['../../../protos/example.proto'])


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

    dauth-shim.cpp

    sbi-path.c
    ausf-sm.c

    init.c
'''.split())

libausf_sources += generated

libausf = static_library('ausf',
    sources : libausf_sources,
    dependencies : [libapp_dep,
                    libcrypt_dep,
                    libsbi_dep,
                    grpc_dep,
                    proto_dep],
    install : false)

libausf_dep = declare_dependency(
    link_with : libausf,
    dependencies : [libapp_dep,
                    libcrypt_dep,
                    libsbi_dep])

ausf_sources = files('''
    app.c
    ../main.c
'''.split())

executable('open5gs-ausfd',
    sources : ausf_sources,
    c_args : '-DDEFAULT_CONFIG_FILENAME="@0@/ausf.yaml"'.format(open5gs_sysconfdir),
    include_directories : srcinc,
    dependencies : [libausf_dep, grpc_dep, proto_dep],
    install_rpath : libdir,
    install : true)
