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

grpc_deps = [
    # Only using the unsecure versions for now to avoid needing to link in heavy
    # crypto/ssl/tls libraries.
    grpc_proj.dependency('grpc_unsecure'),
    grpc_proj.dependency('grpc++_unsecure'),
    # grpc_proj.dependency('grpc'),
    # grpc_proj.dependency('grpc++'),
    grpc_proj.dependency('libprotobuf'),
    grpc_proj.dependency('libprotobuf-lite'),
    grpc_proj.dependency('gpr'),
    grpc_proj.dependency('absl_hash'),
    grpc_proj.dependency('absl_city'),
    grpc_proj.dependency('absl_wyhash'),
    grpc_proj.dependency('absl_raw_hash_set'),
    grpc_proj.dependency('absl_hashtablez_sampler'),
    grpc_proj.dependency('absl_synchronization'),
    grpc_proj.dependency('absl_graphcycles_internal'),
    grpc_proj.dependency('absl_time'),
    grpc_proj.dependency('absl_civil_time'),
    grpc_proj.dependency('absl_time_zone'),
    grpc_proj.dependency('absl_exponential_biased'),
    grpc_proj.dependency('absl_statusor'),
    grpc_proj.dependency('absl_status'),
    grpc_proj.dependency('absl_str_format_internal'),
    grpc_proj.dependency('absl_stacktrace'),
    grpc_proj.dependency('absl_symbolize'),
    grpc_proj.dependency('absl_malloc_internal'),
    grpc_proj.dependency('absl_debugging_internal'),
    grpc_proj.dependency('absl_demangle_internal'),
    grpc_proj.dependency('absl_cord'),
    grpc_proj.dependency('absl_bad_optional_access'),
    grpc_proj.dependency('absl_strings'),
    grpc_proj.dependency('absl_throw_delegate'),
    grpc_proj.dependency('absl_strings_internal'),
    grpc_proj.dependency('absl_base'),
    grpc_proj.dependency('absl_spinlock_wait'),
    grpc_proj.dependency('absl_int128'),
    grpc_proj.dependency('absl_bad_variant_access'),
    grpc_proj.dependency('absl_raw_logging_internal'),
    grpc_proj.dependency('absl_log_severity'),
    grpc_proj.dependency('upb'),
    grpc_proj.dependency('address_sorting'),
    grpc_proj.dependency('c-ares'),
    dependency('zlib')
]

protoc_executable = grpc_proj.target('protoc')
grpc_cpp_plugin_dep = grpc_proj.target('grpc_cpp_plugin')

gen = generator(protoc_executable,
  output    : ['@BASENAME@.pb.cc', '@BASENAME@.pb.h', '@BASENAME@.grpc.pb.cc', '@BASENAME@.grpc.pb.h'],
  depends: grpc_cpp_plugin_dep,
  arguments : [
    '--proto_path=../../protos',
    '--cpp_out=@BUILD_DIR@',
    '--grpc_out=@BUILD_DIR@',
    '--plugin=protoc-gen-grpc=subprojects/grpc/grpc_cpp_plugin',
    '@PLAINNAME@'])

generated = gen.process(['../../../protos/local_authentication.proto'])


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
