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

grpc_sources = gen.process([
    '../../../protos/local_authentication.proto',
    '../../../protos/authentication_data.proto'])
