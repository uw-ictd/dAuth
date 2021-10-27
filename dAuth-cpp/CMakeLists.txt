cmake_minimum_required(VERSION 3.10)

project("dAuthCPP" LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 11)

include(FetchContent)
FetchContent_Declare(
    gRPC
    GIT_REPOSITORY https://github.com/grpc/grpc
    GIT_TAG        v1.37.0
)
set(FETCHCONTENT_QUIET OFF)
FetchContent_MakeAvailable(gRPC)
message(STATUS "Using gRPC ${gRPC_VERSION}")

# Protobuf
# compile example.proto
set(PROTO_DIR "${CMAKE_SOURCE_DIR}/proto")
message(STATUS "Using PROTO_DIR at ${PROTO_DIR}")
message(STATUS "Using Protobuf at ${protobuf_BINARY_DIR}")
message(STATUS "Using gRPC CPP Plugin at ${grpc_BINARY_DIR}")

add_custom_command(
    OUTPUT ${CMAKE_SOURCE_DIR}/example.pb.cc
           ${CMAKE_SOURCE_DIR}/example.pb.h
           ${CMAKE_SOURCE_DIR}/example.grpc.pb.cc
           ${CMAKE_SOURCE_DIR}/example.grpc.pb.h
    COMMAND ${protobuf_BINARY_DIR}/protoc
    ARGS --grpc_out "${CMAKE_SOURCE_DIR}/"
         --cpp_out "${CMAKE_SOURCE_DIR}/"
         -I "${PROTO_DIR}"
         --plugin=protoc-gen-grpc=${grpc_BINARY_DIR}/grpc_cpp_plugin
    example.proto
)

link_directories(${protobuf_BINARY_DIR}/lib)

add_library(proto STATIC
    ${CMAKE_SOURCE_DIR}/example.pb.cc
    ${CMAKE_SOURCE_DIR}/example.pb.h
    ${CMAKE_SOURCE_DIR}/example.grpc.pb.cc
    ${CMAKE_SOURCE_DIR}/example.grpc.pb.h
)
target_link_libraries(proto PUBLIC grpc++ grpc++_reflection)

