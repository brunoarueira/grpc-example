# grpc-example

Simple project to test grpc between ruby (client) and rust (server).

# Usage

To simplify a lot this project uses docker and `docker compose` to build
dependencies and creates the link between each service.

Run the following command:

```sh
$ docker-compose build
```

After build the images from the client and server, run the following:

```sh
$ docker-compose run grpc-client ruby -Ilib/ client.rb
```

# Tips

If you want to modify the protocol buffers definition file, you'll need to
regenerate the generated code from ruby side with the following command:

```sh
$ docker-compose run grpc-client grpc_tools_ruby_protoc -I ../protos --ruby_out=./lib --grpc_out=./lib ../protos/calculator.proto
```

From rust side it'll require a new image build with:

```sh
$ docker-compose build grpc-server
```

After that, kill the services and start again using the last command from usage
section.
