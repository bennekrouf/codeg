
#!/bin/bash

# Set the server address and port
SERVER_ADDRESS="localhost:50056"

# Path to the .proto file
PROTO_FILE_PATH="proto-definitions/codeg.proto"

# The fully qualified name of the service and method
SERVICE_METHOD="codeg.CodeGenerator/GenerateFiles"

# Test the gRPC service using grpcurl
grpcurl -plaintext -proto $PROTO_FILE_PATH $SERVER_ADDRESS $SERVICE_METHOD
