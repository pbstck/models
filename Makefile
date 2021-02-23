all: go_proto rust_proto


go_proto: _setup
	protoc --go_out=./ protobuf/requests.proto

rust_proto:
	cd rust && cargo build

_setup:
ifeq (, $(shell which protoc))
  	$(error "No protoc in $(PATH)")
endif
ifeq (, $(shell which protoc-gen-go))
	go get -u github.com/golang/protobuf/protoc-gen-go
endif

.PHONY: all go_proto rust_proto _setup
