all: build


build: go_proto rust_proto

go_proto:
	protoc --go_out=./ protobuf/requests.proto

rust_proto:
	cd rust && cargo build

_setup:
ifeq (, $(shell which protoc))
  	$(error "No protoc in $(PATH)")
endif
ifeq (, $(shell which protoc-gen-go))
	$(error "No protoc-gen-go in $(PATH), install it using 'go get -u github.com/golang/protobuf/protoc-gen-go'")
endif
