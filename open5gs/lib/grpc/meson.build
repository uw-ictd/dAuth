grpc_deps = [
    dependency('gRPC'),
    dependency('grpc++_unsecure'),
    dependency('grpc_unsecure'),
    dependency('protobuf'),
    dependency('zlib')
]

protoc_executable = find_program('protoc')

gen = generator(protoc_executable,
  output    : ['@BASENAME@.pb.cc', '@BASENAME@.pb.h', '@BASENAME@.grpc.pb.cc', '@BASENAME@.grpc.pb.h'],
  arguments : [
    '--proto_path=../../protos',
    '--cpp_out=@BUILD_DIR@',
    '--grpc_out=@BUILD_DIR@',
    '--plugin=protoc-gen-grpc=/usr/local/bin/grpc_cpp_plugin',
    '@PLAINNAME@'])

grpc_sources = gen.process([
    '../../../protos/local_authentication.proto',
    '../../../protos/authentication_data.proto'])
