# protoc is stupid, so we need to go up a directory to generate the code
# Otherwise imports won't work (it uses 'import ...' rather than 'from network.protos import ...')
cd .. && python3 -m grpc_tools.protoc -I . --python_out=. --grpc_python_out=. network/protos/*.proto